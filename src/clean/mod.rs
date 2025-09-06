//! 自动清理
use log::debug;

use crate::config::PmbsConfigKeep;

mod ls;
mod safe_rm_subvol;

pub use ls::{Snapshot, get_re_t, get_re_year, ls_snapshot};
pub use safe_rm_subvol::{get_re_safe_check_path, safe_rm_subvol_list};

/// 保留规则生成器, 一次输出一条待使用的规则 (间隔时间/秒)
#[derive(Debug, Clone)]
struct KeepIter {
    // 保留规则集
    rule: Vec<PmbsConfigKeep>,
    // 当前使用的规则序号
    i: usize,
    // 当前规则
    r: Option<PmbsConfigKeep>,
}

impl KeepIter {
    /// 创建
    pub fn new(rule: Vec<PmbsConfigKeep>) -> Self {
        let r = if rule.len() > 0 {
            Some(rule[0].clone())
        } else {
            None
        };

        Self { rule, i: 0, r }
    }

    /// 检查加载下一条规则
    fn check_load_next_rule(&mut self) {
        match &mut self.r {
            Some(r) => {
                if r.n < 1 {
                    // 本条规则已用完
                    self.r = None;
                    self.check_load_next_rule();
                }
            }
            None => {
                // 尝试加载下一条规则
                if self.rule.len() > (self.i + 1) {
                    self.i += 1;
                    self.r = Some(self.rule[self.i].clone());
                }
            }
        }
    }
}

impl Iterator for KeepIter {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        self.check_load_next_rule();

        match &mut self.r {
            Some(r) => {
                // 消耗一条规则
                r.n -= 1;

                Some(r.s)
            }
            // 规则已用完
            None => None,
        }
    }
}

/// 决定 保留/清理 快照 (自动清理核心算法)
///
/// 返回: (保留列表, 清理列表)
pub fn decide(
    rule: Vec<PmbsConfigKeep>,
    mut snapshot: Vec<Snapshot>,
) -> (Vec<Snapshot>, Vec<Snapshot>) {
    fn debug_snapshot_list(list: &Vec<Snapshot>) -> String {
        format!("{:?}", list.iter().map(|x| x.t).collect::<Vec<_>>())
    }

    debug!("rule  {:?}", rule);

    // 排序 (按时间降序, 最新的在最前面)
    snapshot.sort_by(|a, b| b.t.cmp(&a.t));

    debug!("snapshot  {}", debug_snapshot_list(&snapshot));
    // 保留规则生成器
    let mut ki = KeepIter::new(rule);
    // 保留的快照列表
    let mut keep: Vec<Snapshot> = Vec::new();
    // 清理的快照列表
    let mut clean: Vec<Snapshot> = Vec::new();

    // 硬编码: 最新快照之前 5 分钟的快照, 全部保留
    const KEEP_LATEST: u64 = 300;
    // 容忍系统时间误差: 时间检查减少 10 秒
    const KEEP_S: u64 = 10;

    if snapshot.len() > 0 {
        // 当前保留规则
        let mut rule = ki.next();
        // 基准时间戳: 当前的最新快照 (硬编码保留)
        let t0 = snapshot[0].t - KEEP_LATEST;
        // 当前保留时间戳
        let mut t = t0;

        // 决定每个快照的命运
        for i in snapshot {
            // 硬编码: 最新快照之前 5 分钟的快照, 全部保留
            if (i.t + KEEP_S) > t0 {
                keep.push(i.clone());
                t = i.t;
                continue;
            }
            // 检查规则遮盖
            match rule {
                Some(s) => {
                    if i.t > (t - s + KEEP_S) {
                        // 被遮盖, 丢弃
                        clean.push(i.clone());
                    } else {
                        // 未被遮盖, 消耗一条规则
                        keep.push(i.clone());
                        t = i.t;
                        rule = ki.next();
                    }
                }
                None => {
                    // 规则用尽, 全部丢弃
                    clean.push(i.clone());
                }
            }
        }
    }
    // 排序 (清理应该从最旧的开始)
    clean.sort_by(|a, b| a.t.cmp(&b.t));

    debug!("keep  {}", debug_snapshot_list(&keep));
    debug!("clean  {}", debug_snapshot_list(&clean));
    (keep, clean)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn keep_iter() {
        let mut ki = KeepIter::new(vec![
            PmbsConfigKeep::new_sn(5, 3),
            PmbsConfigKeep::new_sn(20, 2),
            PmbsConfigKeep::new_sn(60, 1),
        ]);
        assert_eq!(ki.next(), Some(5));
        assert_eq!(ki.next(), Some(5));
        assert_eq!(ki.next(), Some(5));
        assert_eq!(ki.next(), Some(20));
        assert_eq!(ki.next(), Some(20));
        assert_eq!(ki.next(), Some(60));
        assert_eq!(ki.next(), None);
        assert_eq!(ki.next(), None);
    }
}
