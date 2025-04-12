use darling::ast::NestedMeta;

use crate::utils::error::WithSpan;
use crate::utils::meta_match::MatchNestedMeta;
use crate::utils::meta_match::match_nested_meta_list::MatchNestedMetaList;

impl<T1: MatchNestedMeta> MatchNestedMetaList for (T1,) {
    fn match_nested_meta_list(list: &[NestedMeta]) -> Option<darling::Result<Self>> {
        if list.len() > 1 {
            return Some(Err(darling::Error::too_many_items(1)).with_span(&list[1]));
        }

        let [t1] = list else {
            return Some(Err(darling::Error::too_few_items(1)).with_span(&list[0]));
        };
        let (t1,) = (T1::match_nested_meta(t1)?,);
        let r: darling::Result<_> = (|| Ok((t1?,)))();

        Some(r)
    }
}

impl<T1: MatchNestedMeta, T2: MatchNestedMeta> MatchNestedMetaList for (T1, T2) {
    fn match_nested_meta_list(list: &[NestedMeta]) -> Option<darling::Result<Self>> {
        if list.len() > 2 {
            return Some(Err(darling::Error::too_many_items(2)).with_span(&list[2]));
        }
        let [t1, t2] = list else {
            return Some(Err(darling::Error::too_few_items(2)).with_span(&list[0]));
        };
        let (t1, t2) = (T1::match_nested_meta(t1)?, T2::match_nested_meta(t2)?);
        let r: darling::Result<_> = (|| Ok((t1?, t2?)))();

        Some(r)
    }
}
