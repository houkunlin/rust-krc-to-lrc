#[cfg(test)]
mod tests {
    use std::fs;
    use crate::krc_to_lrc::{krc_to_lrc, replace_krc_time, str_to_time};

    #[test]
    fn test_str_to_time() {
        assert_eq!(str_to_time(61234), "[01:01.23]");
        assert_eq!(str_to_time(52590), "[00:52.59]");
    }

    #[test]
    fn test_replace_krc_time() {
        assert_eq!(replace_krc_time("[61234,2000]这里是歌词内容", 500), "[00:00.00]\n[01:01.23]这里是歌词内容\n");
    }

    #[test]
    fn test_krc_to_lrc() {
        let string = krc_to_lrc(fs::read("2.krc").unwrap(), 500);
        assert_eq!(string.contains("[00:00.00]张韶涵 - 亲爱的，那不是爱情"), true);
    }
}
