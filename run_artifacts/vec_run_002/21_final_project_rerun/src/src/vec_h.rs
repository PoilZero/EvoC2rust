use crate::translation_utils::*;
pub use crate::src::vec_c::vec_reserve_;
pub use crate::src::vec_c::vec_swap_;
pub use crate::src::vec_c::vec_swapsplice_;
pub use crate::src::vec_c::vec_reserve_po2_;
pub use crate::src::vec_c::vec_compact_;
pub use crate::src::vec_c::vec_expand_;
pub use crate::src::vec_c::vec_splice_;
pub use crate::src::vec_c::vec_insert_;

pub type VecVoidT = VecPtr<VoidPtr>;

pub type vec_str_t = Vec<Ptr<u8>>;

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct VecInt {
    pub data: Ptr<i32>,
    pub length: u32,
}
pub type VecIntT = VecInt;

pub type vec_char_t = Vec<u8>;

pub type VecFloatT = Array<f32, 1>;

pub type VecDoubleT = FuncPtr<fn(f64) -> VecT>;

macro_rules! VEC_H { () => {} }
pub(crate) use VEC_H;

macro_rules! VEC_VERSION { () => { cstr!("0.2.1") } }
pub(crate) use VEC_VERSION;

macro_rules! vec_unpack_ {
    ($v:expr) => {
        (
            c_ref!($v.data).cast::<Ptr<Ptr<u8>>>(),
            c_ref!($v.length),
            c_ref!($v.capacity),
            c_sizeof!(u8)
        )
    }
}
pub(crate) use vec_unpack_;

pub(crate) macro_rules! vec_t {
    ($t:ty) => {
        struct Vec {
            data: Ptr<$t>,
            length: i32,
            capacity: i32,
        }
    };
}
pub(crate) use vec_t;

macro_rules! vec_init {
    ($v:expr) => {
        memset($v.cast(), 0, c_sizeofval!(*$v))
    }
}
pub(crate) use vec_init;

pub(crate) macro_rules! vec_deinit {
    ($v:expr) => {
        unsafe {
            free($v.data.cast::<VoidPtr>());
        }
        vec_init($v);
    }
}

macro_rules! vec_push {
    ($v:expr, $val:expr) => {
        if vec_expand_(vec_unpack_($v.cast())) {
            -1
        } else {
            let idx = $v.length;
            $v.data[idx] = $val as u8;
            $v.length.suffix_plus_plus();
            0
        }
    }
}
pub(crate) use vec_push;

macro_rules! vec_pop {
    ($v:expr) => {
        let idx = minus_minus!($v.length);
        $v.data[idx]
    }
}
pub(crate) use vec_pop;

macro_rules! vec_splice {
    ($v:expr, $start:expr, $count:expr) => {
        vec_splice_(vec_unpack_($v), $start, $count);
        $v.length -= $count;
    }
}
pub(crate) use vec_splice;

macro_rules! vec_swapsplice { ($v:expr, $start:expr, $count:expr) => {
    vec_swapsplice_(vec_unpack_($v), $start, $count);
    $v.length -= $count;
} }
pub(crate) use vec_swapsplice;

macro_rules! vec_insert {
    ($v:expr, $idx:expr, $val:expr) => {
        let unpacked_v = vec_unpack_!($v);
        let result = vec_insert_(unpacked_v, $idx);
        if result != 0 {
            -1
        } else {
            let ptr = unpacked_v + $idx;
            c_ref!(ptr) = $val;
            $v.length.suffix_plus_plus();
            0
        }
        $v.length.suffix_plus_plus();
        0
    }
}
pub(crate) use vec_insert;

pub(crate) macro_rules! vec_sort {
    ($v:expr, $fn:expr) => {
        qsort(
            $v.data.cast(),
            $v.length.cast(),
            c_sizeof!(*$v.data),
            $fn
        )
    }
}

macro_rules! vec_swap {
    ($v:expr, $idx1:expr, $idx2:expr) => {
        vec_swap_!(vec_unpack_!($v).cast(), $idx1.cast(), $idx2.cast())
    }
}
pub(crate) use vec_swap;

pub(crate) macro_rules! vec_truncate {
    ($v:expr, $len:expr) => {
        let v = $v.cast();
        let v_deref = v.deref();
        v_deref.length = if $len < v_deref.length { $len } else { v_deref.length };
    }
}
pub(crate) use vec_truncate;

macro_rules! vec_clear {
    ($v:expr) => {
        $v.cast().length = 0;
    };
}
pub(crate) use vec_clear;

macro_rules! vec_first { ($v:expr) => { $v.data[0] } }
pub(crate) use vec_first;

macro_rules! vec_last { ($v:expr) => {
    $v.cast().data[$v.cast().length - 1]
} }
pub(crate) use vec_last;

pub(crate) macro_rules! vec_reserve {
    ($v:expr, $n:expr) => {
        vec_reserve_(vec_unpack_($v.cast()), $n)
    };
}
pub(crate) use vec_reserve;

macro_rules! vec_compact {
    ($v:expr) => {
        vec_compact_(vec_unpack_!($v.cast()))
    }
}
pub(crate) use vec_compact;

macro_rules! vec_pusharr {
    ($v:expr, $arr:expr, $count:expr) => {
        let mut i__: i32;
        let n__ = $count;
        if vec_reserve_po2_(vec_unpack_($v.cast()), $v.length + n__) != 0 {
            // Break statement is handled by the loop context (not needed in macro body)
        }
        for i__ in 0..n__ {
            let idx = $v.length;
            $v.data[idx] = $arr[i__] as u8;
            $v.length = $v.length.plus_plus();
        }
    }
}
pub(crate) use vec_pusharr;

macro_rules! vec_extend {
    ($v:expr, $v2:expr) => {
        vec_pusharr(
            $v.cast(),
            c_ref!($v2.data),
            $v2.length.cast()
        );
    }
}
pub(crate) use vec_extend;

macro_rules! vec_find { ($v:expr, $val:expr, $idx:expr) => {
    let mut i: i32 = 0;
    while i < $v.length {
        if $v.data[i] == $val {
            break;
        }
        i.suffix_plus_plus();
    }
    if i == $v.length {
        $idx = -1;
    }
} }
pub(crate) use vec_find;

macro_rules! vec_remove { ($v:expr, $val:expr) => {
    {
        let mut idx__: i32;
        vec_find($v.cast(), $val.cast(), idx__.cast());
        if idx__ != -1 {
            vec_splice($v.cast(), idx__.cast(), 1);
        }
    }
} }

pub(crate) use vec_remove;

macro_rules! vec_reverse { ($v:expr) => {
    {
        let mut i__: i32 = ($v).length / 2;
        while i__ > 0 {
            vec_swap($v, i__, (($v).length - (i__ + 1)));
            i__.minus_minus();
        }
    }
} }
pub(crate) use vec_reverse;

macro_rules! vec_foreach { ($v:expr, $var:expr, $iter:expr) => {
    if $v.length > 0 {
        c_for!($iter = 0; $iter < $v.length; $iter.plus_plus(); {
            $var = $v.data[$iter];
        })
    }
} }
pub(crate) use vec_foreach;

macro_rules! vec_foreach_rev { ($v:expr, $var:expr, $iter:expr) => {
    if $v.length > 0 {
        let mut $iter = $v.length - 1;
        while $iter >= 0 {
            let $var = $v.data[$iter];
            $iter -= 1;
        }
    }
} }
pub(crate) use vec_foreach_rev;

pub(crate) macro_rules! vec_foreach_ptr {
    ($v:expr, $var:expr, $iter:expr) => {
        if $v.length > 0 {
            for iter in 0..$v.length {
                $var = c_ref!($v.data[iter]).cast::<Ptr<u8>>();
            }
        }
    };
}
pub(crate) use vec_foreach_ptr;

macro_rules! vec_foreach_ptr_rev {
    ($v:expr, $var:expr, $iter:expr) => {
        if $v.length > 0 {
            let mut iter = $v.length - 1;
            while iter >= 0 {
                $var = c_ref!($v.data[iter]);
                iter = iter.minus_minus();
            }
        }
    };
}
pub(crate) use vec_foreach_ptr_rev;

