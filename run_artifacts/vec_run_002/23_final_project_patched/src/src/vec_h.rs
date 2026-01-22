use crate::translation_utils::*;
pub use crate::src::vec_c::vec_reserve_po2_;
pub use crate::src::vec_c::vec_reserve_;
pub use crate::src::vec_c::vec_insert_;
pub use crate::src::vec_c::vec_swap_;
pub use crate::src::vec_c::vec_swapsplice_;
pub use crate::src::vec_c::vec_splice_;
pub use crate::src::vec_c::vec_expand_;
pub use crate::src::vec_c::vec_compact_;

pub type VecVoidT = Vec<VoidPtr>;

pub type VecStrT = Vec<Ptr<u8>>;

pub type VecIntT = Vec<i32>;

pub type VecCharT = Vec<u8>;

pub type VecFloatT = Vec<f32>;

pub type VecDoubleT = Vec<f64>;

macro_rules! VEC_H { () => { } }
pub(crate) use VEC_H;

macro_rules! VEC_VERSION { () => { cstr!("0.2.1") } }
pub(crate) use VEC_VERSION;

macro_rules! vec_unpack_ { ($v:expr) => { c_ref!($v.data).cast::<Ptr<u8>>(), c_ref!($v.length), c_ref!($v.capacity), c_sizeofval!($v.data) } }
pub(crate) use vec_unpack_;

macro_rules! vec_t {
    ($T:ty) => {
        struct {
            data: Ptr<$T>,
            length: i32,
            capacity: i32,
        }
    };
}
pub(crate) use vec_t;

macro_rules! vec_init { ($v:expr) => { c_memset!($v.cast(), 0, c_sizeofval!(*$v).cast()) } }
pub(crate) use vec_init;

macro_rules! vec_deinit { ($v:expr) => { free!($v.data); vec_init!($v) } }
pub(crate) use vec_deinit;

macro_rules! vec_push { ($v:expr, $val:expr) => 
    {
        if vec_expand_(vec_unpack_($v)) {
            -1
        } else {
            $v.data[$v.length++] = $val;
            0
        }
    }
}
pub(crate) use vec_push;

macro_rules! vec_pop { ($v:expr) => { $v.data[($v).length.minus_minus()] } }
pub(crate) use vec_pop;

macro_rules! vec_splice { ($v:expr, $start:expr, $count:expr) => 
    {
        vec_splice_(vec_unpack_!($v), $start, $count);
        $v.length -= $count;
    }
}
pub(crate) use vec_splice;

macro_rules! vec_swapsplice { ($v:expr, $start:expr, $count:expr) => 
    {
        vec_swapsplice_(vec_unpack_!($v), $start, $count);
        $v.length -= $count;
    }
}
pub(crate) use vec_swapsplice;

macro_rules! vec_insert {
    ($v:expr, $idx:expr, $val:expr) => {
        if vec_insert_(vec_unpack_!($v), $idx) != 0 {
            -1
        } else {
            $v.data[$idx] = $val;
            $v.length.plus_plus();
            0
        }
    }
}
pub(crate) use vec_insert;

macro_rules! vec_sort { ($v:expr, $fn:expr) => { qsort($v.data.cast(), $v.length.cast(), c_sizeofval!($v.data), $fn.cast()) } }
pub(crate) use vec_sort;

macro_rules! vec_swap { ($v:expr, $idx1:expr, $idx2:expr) => { vec_swap_(vec_unpack_!($v), $idx1, $idx2) } }
pub(crate) use vec_swap;

macro_rules! vec_truncate { ($v:expr, $len:expr) => { $v.length = if $len < $v.length { $len } else { $v.length } } }
pub(crate) use vec_truncate;

macro_rules! vec_clear { ($v:expr) => { $v.length = 0 } }
pub(crate) use vec_clear;

macro_rules! vec_first { ($v:expr) => { $v.data[0] } }
pub(crate) use vec_first;

macro_rules! vec_last { ($v:expr) => { $v.data[($v).length - 1] } }
pub(crate) use vec_last;

macro_rules! vec_reserve { ($v:expr, $n:expr) => { vec_reserve_(vec_unpack_!($v), $n) } }
pub(crate) use vec_reserve;

macro_rules! vec_compact { ($v:expr) => { vec_compact_(vec_unpack_!($v)) } }
pub(crate) use vec_compact;

macro_rules! vec_pusharr {
    ($v:expr, $arr:expr, $count:expr) => {
        let i__: i32;
        let n__: i32 = $count;
        if vec_reserve_po2_!(vec_unpack_!($v), ($v).length + n__) != 0 {
            break;
        }
        for i__ in 0..n__ {
            ($v).data[($v).length] = $arr[i__];
            ($v).length += 1;
        }
    }
}
pub(crate) use vec_pusharr;

macro_rules! vec_extend { ($v:expr, $v2:expr) => { vec_pusharr($v.cast(), $v2.cast().data, $v2.cast().length) } }
pub(crate) use vec_extend;

macro_rules! vec_find {
    ($v:expr, $val:expr, $idx:expr) => {
        {
            let mut $idx = 0;
            while $idx < $v.length {
                if $v.data[$idx] == $val {
                    break;
                }
                $idx.plus_plus();
            }
            if $idx == $v.length {
                $idx = -1;
            }
        }
    }
}
pub(crate) use vec_find;

macro_rules! vec_remove { ($v:expr, $val:expr) =>
    {
        let mut idx__: i32;
        vec_find!($v, $val, idx__);
        if idx__ != -1 {
            vec_splice!($v, idx__, 1);
        }
    }
}
pub(crate) use vec_remove;

macro_rules! vec_reverse { ($v:expr) =>
    {
        let mut i__ = ($v).length / 2;
        while i__ > 0
        {
            vec_swap!($v, i__, ($v).length - (i__ + 1));
            i__.minus_minus();
        }
    }
}
pub(crate) use vec_reverse;

macro_rules! vec_foreach {
    ($v:expr, $var:ident, $iter:ident) => {
        if $v.length > 0 {
            let mut $iter = 0;
            while $iter < $v.length && {
                $var = $v.data[$iter];
                true
            } {
                $iter.plus_plus();
            }
        }
    }
}
pub(crate) use vec_foreach;

macro_rules! vec_foreach_rev {
    ($v:expr, $var:expr, $iter:expr) => {
        if $v.length > 0 {
            let mut $iter = $v.length - 1;
            while $iter >= 0 && {
                $var = $v.data[$iter];
                true
            } {
                $iter.minus_minus();
            }
        }
    }
}
pub(crate) use vec_foreach_rev;

macro_rules! vec_foreach_ptr {
    ($v:expr, $var:ident, $iter:ident) => {
        if $v.length > 0 {
            let mut $iter = 0;
            while $iter < $v.length && {
                $var = c_ref!($v.data[$iter]);
                true
            } {
                $iter.plus_plus();
            }
        }
    }
}
pub(crate) use vec_foreach_ptr;

macro_rules! vec_foreach_ptr_rev {
    ($v:expr, $var:expr, $iter:expr) => {
        if $v.length > 0 {
            let mut $iter = $v.length - 1;
            while $iter >= 0 && {
                $var = c_ref!($v.data[$iter]);
                true
            } {
                $iter.minus_minus();
            }
        }
    }
}
pub(crate) use vec_foreach_ptr_rev;

