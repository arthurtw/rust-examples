use std::cmp::Ordering::{Less, Greater, Equal};

#[derive(Show)]
pub struct BTreeMap<K: Ord, V> {
    root: Link<K, V>,
    length: usize,
}

type Link<K, V> = Option<Box<TreeNode<K, V>>>;

#[derive(Show)]
struct TreeNode<K: Ord, V> {
    left: Link<K, V>,
    right: Link<K, V>,
    key: K,
    value: V,
}

impl<K: Ord, V> BTreeMap<K, V> {

    pub fn new() -> BTreeMap<K, V> {
        BTreeMap { root: None, length: 0 }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let mut link = &self.root;
        loop {
            match *link {
                Some(ref node) =>
                    match key.cmp(&node.key) {
                        Less => link = &node.left,
                        Greater => link = &node.right,
                        Equal => return Some(&node.value),
                    },
                None => return None,
            }
        }
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        let mut link = &mut self.root;
        loop {
            let temp = link; // to appease borrowck (see std lib TreeMap src)
            match *temp {
                Some(ref mut node) =>
                    match key.cmp(&node.key) {
                        Less => link = &mut node.left,
                        Greater => link = &mut node.right,
                        Equal => return Some(&mut node.value),
                    },
                None => return None,
            }
        }
    }

    pub fn iter(&self) -> Entries<K, V> {
        let mut v = Vec::new();
        collect(&self.root, &mut v);
        Entries { entries: v }
    }

    pub fn insert(&mut self, key: K, value: V) -> bool {
        let ret = insert(&mut self.root, key, value);
        if ret {
            self.length += 1;
        }
        ret
    }
}

pub struct Entries<'a, K: 'a, V: 'a> {
    entries: Vec<(&'a K, &'a V)>,
}

impl<'a, K, V> Iterator for Entries<'a, K, V> {
    type Item = (&'a K, &'a V);
    fn next(&mut self) -> Option<(&'a K, &'a V)> {
        self.entries.pop()
    }
}

fn insert<K: Ord, V>(link: &mut Link<K, V>, key: K, value: V) -> bool {
    match *link {
        Some(ref mut node) =>
            match key.cmp(&node.key) {
                Less => return insert(&mut node.left, key, value),
                Greater => return insert(&mut node.right, key, value),
                Equal => { node.value = value; false }
            },
        None => {
            *link = Some(Box::new(TreeNode {
                left: None,
                right: None,
                key: key,
                value: value,
            }));
            true
        }
    }
}

fn collect<'a, K: Ord, V>(link: &'a Link<K, V>, result: &mut Vec<(&'a K, &'a V)>) {
    match *link {
        Some(ref node) => {
            // collect in reverse order for later pop() use
            collect(&node.right, result);
            result.push((&node.key, &node.value));
            collect(&node.left, result);
        }
        None => {}
    }
}

#[cfg(test)]
mod test {
    use super::BTreeMap;

    #[test]
    fn test_new() {
        let map: BTreeMap<&str, i32> = BTreeMap::new();
        assert_eq!(map.len(), 0);
    }

    #[test]
    fn test_update() {
        let mut map = BTreeMap::new();

        map.insert("b", 3);
        // b:3
        assert_eq!(map.len(), 1);

        map.insert("c", 4);
        map.insert("b", 5); // overwrite
        map.insert("a", 6);
        // a:6, b:5, c:4
        assert_eq!(map.len(), 3);
        assert_eq!(map.get(&"x"), None);
        assert_eq!(map.get(&"a"), Some(&6));

        {
            let value = map.get_mut(&"b").unwrap();
            assert_eq!(*value, 5);
            *value *= -1;
        }
        // a:6, b:-5, c:4
        assert_eq!(map.get(&"b"), Some(&-5));
        let items = map.iter().collect::<Vec<(&&str, &i32)>>();
        assert_eq!(items[0].0, &"a");
        assert_eq!(items[2].1, &4);
    }
}

