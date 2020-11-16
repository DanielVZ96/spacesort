use std::collections::{HashSet, HashMap};
use std::hash::Hash;
use std::fmt::{Debug, Display};
extern crate num;
extern crate quickersort;
use num::{FromPrimitive, ToPrimitive};

const MAX_QUICKER_SORT_ELEMENTS: usize = 50;
const MIN_HAS_DUP_SETTABILITY: i64 = 9000;
const MIN_RANGE_DERANGE: i64 = 500_000;

pub trait SpaceSortable:
    Hash + ToPrimitive + FromPrimitive + Ord + PartialOrd + Clone + Copy
{
}

impl<T> SpaceSortable for T where
    T: Hash + ToPrimitive + FromPrimitive + Ord + PartialOrd + Clone + Copy
{
}

pub fn space_sort<T: SpaceSortable>(v: Vec<T>) -> Vec<T> {
    let len = v.len();
    if len <= MAX_QUICKER_SORT_ELEMENTS {
        let mut r: Vec<T> = v.clone();
        quickersort::sort(&mut r);
        return r;
    }

    let max = v.iter().max().unwrap().to_i64().unwrap();
    let min = v.iter().min().unwrap().to_i64().unwrap();
    let settability_index = 1 - (max / len as i64);
    let mut is_set = false;
    if settability_index > MIN_HAS_DUP_SETTABILITY {
        is_set = !has_dup(&v);
    }

    if is_set {
        return space_sort_set(v, max);
    } else if max - min > MIN_RANGE_DERANGE || min < 0 {
        return space_sort_not_set_deranged(v, len, min, max);
    } else {
        return space_sort_not_set(v, len, min, max);
    }
}

pub fn space_sort_by<T: SpaceSortable + Debug + Display, U: Fn(V) -> T, V: Copy + Debug + Display>(v: Vec<V>, key: U) -> Vec<V> {
    let len = v.len();
    let mut value_map : HashMap<T, V> = HashMap::with_capacity(len);
    for val in v.iter() {
        value_map.insert(key(*val), *val);
    }
    return space_sort(value_map.keys().map(|x|*x).collect()).iter()
        .map(|x| value_map[x])
        .collect()
}

pub fn has_dup<T: SpaceSortable>(v: &Vec<T>) -> bool {
    let mut set: HashSet<T> = HashSet::with_capacity(v.len());
    for i in v.iter() {
        if set.contains(i) {
            return true;
        }
        set.insert(*i);
    }
    return false;
}
pub fn space_sort_not_set<T: SpaceSortable>(v: Vec<T>, n: usize, _min: i64, max: i64) -> Vec<T> {
    let mut index: Vec<usize> = vec![0; (max + 1) as usize];
    for i in v.iter() {
        index[i.to_usize().unwrap()] += 1;
    }
    let mut res: Vec<T> = vec![T::from_i8(0).unwrap(); n];
    let mut z: usize = 0;
    for (v, &count) in index.iter().enumerate() {
        for _ in 0..count {
            res[z] = T::from_usize(v as usize).unwrap();
            z += 1;
        }
    }
    return res;
}

pub fn space_sort_not_set_deranged<T: SpaceSortable>(
    v: Vec<T>,
    n: usize,
    min: i64,
    max: i64,
) -> Vec<T> {
    let mut index: Vec<i64> = vec![0; (max - min + 1) as usize];
    for i in v.iter() {
        index[i.to_usize().unwrap() - min as usize] += 1;
    }
    let mut res: Vec<T> = vec![T::from_i8(0).unwrap(); n];
    let mut z: usize = 0;
    for (v, &count) in index.iter().enumerate() {
        for _ in 0..count {
            res[z] = T::from_usize(v + min as usize).unwrap();
            z += 1;
        }
    }
    return res;
}

pub fn space_sort_set<T: SpaceSortable>(v: Vec<T>, max: i64) -> Vec<T> {
    let mut index: Vec<bool> = vec![false; max.to_usize().unwrap() + 1];

    for i in v.iter() {
        index[i.to_usize().unwrap()] = true;
    }

    return index
        .into_iter()
        .enumerate()
        .filter(|(_, x)| *x)
        .map(|(i, _)| T::from_usize(i).unwrap())
        .collect();
}


//pub fn sort_v1(v: Vec<usize>) -> Vec<usize> {
//    let sum: usize = v.iter().sum();
//    let mut index: Vec<Option<usize>> = vec![None; sum];
//    for i in v.iter() {
//        index[*i as usize] = Some(*i);
//    }
//    index
//        .iter()
//        .filter(|x| x.is_some())
//        .map(|x| x.unwrap())
//        .collect()
//}
//
//pub fn sort_v2(v: Vec<usize>) -> Vec<usize> {
//    let max = v.iter().max().unwrap();
//
//    let mut index: Vec<bool> = vec![false; max + 1];
//
//    for i in v.iter() {
//        index[*i] = true;
//    }
//
//    return index
//        .into_iter()
//        .enumerate()
//        .filter(|(_, x)| *x)
//        .map(|(i, _)| i)
//        .collect();
//}
//
//pub fn sort_v2_fair(v: Vec<usize>) -> Vec<usize> {
//    const MAX_MIN_TRIM: usize = 10;
//
//    let mut max: usize = *v.iter().max().unwrap();
//    let trim_min = MAX_MIN_TRIM < max;
//
//    let mut min: &usize = &0;
//    if trim_min {
//        min = v.iter().min().unwrap();
//        max -= min;
//    }
//    let mut index: Vec<Option<usize>> = vec![None; max + 1];
//
//    for i in v.iter() {
//        index.insert(i - min, Some(*i));
//    }
//    return index.iter().filter_map(|x| *x).collect();
//}
//pub fn counting_sort(arr: Vec<usize>) -> Vec<usize> {
//    let maxval: usize = *arr.iter().max().unwrap();
//    let mut occurences: Vec<usize> = vec![0; maxval + 1];
//    for &data in arr.iter() {
//        occurences[data] += 1;
//    }
//    let mut i = 0;
//    let mut res = vec![0; arr.len()];
//    for (data, &number) in occurences.iter().enumerate() {
//        for _ in 0..number {
//            res[i] = data;
//            i += 1;
//        }
//    }
//    return res;
//}

//pub fn sort_v2_fair_v2(v: Vec<usize>) -> Vec<usize> {
//    let min = v.iter().min().unwrap();
//    let max = v.iter().max().unwrap() - min; // not real max
//    let can_be_set = max <= v.len();
//    let is_set;
//    if can_be_set {
//        is_set = !has_dup(&v);
//    } else {
//        is_set = false;
//    }
//
//    if is_set {
//        return sort_v2(v);
//    } else {
//        //let mut index : Vec<usize> = vec![0; max+1];
//        let mut index: Vec<usize> = vec![0; max + 1];
//        for i in &v {
//            index[i - min] += 1;
//        }
//        return index
//            .iter()
//            .filter(|x| **x != 0)
//            .enumerate()
//            .map(|(i, c)| vec![i + min; *c])
//            .flatten()
//            .collect();
//    }
//}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort() {
        let v = vec![1, 4, 3, 2];
        assert_eq!(vec![1, 2, 3, 4], space_sort(v));
        let v = vec![4, 3, 2, 1];
        assert_eq!(vec![1, 2, 3, 4], space_sort(v));
    }
    #[test]
    fn test_sort_by() {
        let v = vec!["Aa", "aaaa", "asdhjasd", "c"];
        assert_eq!(vec!["c", "Aa", "aaaa", "asdhjasd"], space_sort_by(v, |s| s.len()));
    }

    #[test]
    fn test_sort_not_set() {
        let v = vec![1, 4, 3, 2];
        assert_eq!(vec![1, 2, 3, 4], space_sort_not_set(v, 4, 1, 4));
        let v = vec![4, 3, 2, 1];
        assert_eq!(vec![1, 2, 3, 4], space_sort_not_set(v, 4, 1, 4));
    }
    //    #[test]
    //    fn test_sort_v1() {
    //        let v = vec![1, 4, 3, 2];
    //        assert_eq!(vec![1, 2, 3, 4], sort_v1(v));
    //        let v = vec![4, 3, 2, 1];
    //        assert_eq!(vec![1, 2, 3, 4], sort_v1(v));
    //    }
    //    #[test]
    //    fn test_sort_v2() {
    //        let v = vec![1, 4, 3, 2];
    //        assert_eq!(
    //            vec![1 as usize, 2 as usize, 3 as usize, 4 as usize],
    //            sort_v2(v)
    //        );
    //        let v = vec![4, 3, 2, 1];
    //        assert_eq!(vec![1, 2, 3, 4], sort_v2(v));
    //    }
    //    #[test]
    //    fn test_sort_v2_fair_v2() {
    //        let v = vec![1, 4, 3, 2];
    //        assert_eq!(
    //            vec![1 as usize, 2 as usize, 3 as usize, 4 as usize],
    //            sort_v2_fair_v2(v)
    //        );
    //        let v = vec![4, 3, 2, 1];
    //        assert_eq!(vec![1, 2, 3, 4], sort_v2_fair_v2(v));
    //        let v = vec![4, 3, 2, 3, 1];
    //        assert_eq!(vec![1, 2, 3, 3, 4], sort_v2_fair_v2(v));
    //    }
    //    #[test]
    //    fn test_sort_v2_fair_v3() {
    //        let v = vec![1, 4, 3, 2];
    //        assert_eq!(
    //            vec![1 as usize, 2 as usize, 3 as usize, 4 as usize],
    //            sort_v2_fair_v2(v)
    //        );
    //        let v = vec![4, 3, 2, 1];
    //        assert_eq!(vec![1, 2, 3, 4], sort_v2_fair_v3(v));
    //        let v = vec![4, 3, 2, 3, 1];
    //        assert_eq!(vec![1, 2, 3, 3, 4], sort_v2_fair_v3(v));
    //        let v = vec![4, 3, 2, 2, 3, 1];
    //        assert_eq!(vec![1, 2, 2, 3, 3, 4], sort_v2_fair_v3(v));
    //    }
}
