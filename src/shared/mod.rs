pub struct Solution {}

#[derive(PartialEq, Eq)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl std::fmt::Debug for ListNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.val)?;

        if let Some(n) = &self.next {
            write!(f, ",{:?}", n)?;
        }

        Ok(())
    }
}

// impl ListNode {
//     #[inline]
//     pub(crate) fn new(val: i32) -> Self {
//         ListNode { next: None, val }
//     }
// }

#[must_use]
pub fn to_ll(input: &[i32]) -> Option<Box<ListNode>> {
    if input.is_empty() {
        return None;
    }

    Some(Box::new(ListNode {
        val: input[0],
        next: to_ll(&input[1..]),
    }))
}

#[must_use]
pub fn vec_eq<T>(left: Vec<T>, mut right: Vec<T>) -> bool
where
    T: std::cmp::Eq,
{
    if left.len() != right.len() {
        return false;
    }

    for l in left {
        if let Some(p) = right.iter().position(|x| x == &l) {
            right.remove(p);
        } else {
            return false;
        }
    }

    true
}

pub fn sort_vec_of_vec<T>(vec: &mut Vec<Vec<T>>)
where
    T: std::cmp::Ord,
{
    for inner_v in vec.iter_mut() {
        inner_v.sort_unstable();
    }

    vec.sort_unstable();
}
