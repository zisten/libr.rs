//! R_ext/Applic.h
//!
//! Code based automatically generated by rust-bindgen.

pub use super::boolean::*;
pub use super::rs::*;
pub use super::blas::*;

pub type integr_fn =
    extern "C" fn(x: *mut ::libc::c_double, n: ::libc::c_int,
                  ex: *mut ::libc::c_void) -> ();
pub type optimfn =
    extern "C" fn(arg1: ::libc::c_int, arg2: *mut ::libc::c_double,
                  arg3: *mut ::libc::c_void) -> ::libc::c_double;
pub type optimgr =
    extern "C" fn(arg1: ::libc::c_int, arg2: *mut ::libc::c_double,
                  arg3: *mut ::libc::c_double, arg4: *mut ::libc::c_void)
        -> ();
pub type fcn_p =
    ::std::option::Option<extern "C" fn(arg1: ::libc::c_int,
                                        arg2: *mut ::libc::c_double,
                                        arg3: *mut ::libc::c_double,
                                        arg4: *mut ::libc::c_void) -> ()>;
pub type d2fcn_p =
    ::std::option::Option<extern "C" fn(arg1: ::libc::c_int,
                                        arg2: ::libc::c_int,
                                        arg3: *mut ::libc::c_double,
                                        arg4: *mut ::libc::c_double,
                                        arg5: *mut ::libc::c_void) -> ()>;
#[link(name = "R")]
extern {
    pub fn Rdqags(f: integr_fn,
                  ex: *mut ::libc::c_void,
                  a: *mut ::libc::c_double,
                  b: *mut ::libc::c_double,
                  epsabs: *mut ::libc::c_double,
                  epsrel: *mut ::libc::c_double,
                  result: *mut ::libc::c_double,
                  abserr: *mut ::libc::c_double,
                  neval: *mut ::libc::c_int,
                  ier: *mut ::libc::c_int,
                  limit: *mut ::libc::c_int,
                  lenw: *mut ::libc::c_int,
                  last: *mut ::libc::c_int,
                  iwork: *mut ::libc::c_int,
                  work: *mut ::libc::c_double)
                  -> ();
    pub fn Rdqagi(f: integr_fn,
                  ex: *mut ::libc::c_void,
                  bound: *mut ::libc::c_double,
                  inf: *mut ::libc::c_int,
                  epsabs: *mut ::libc::c_double,
                  epsrel: *mut ::libc::c_double,
                  result: *mut ::libc::c_double,
                  abserr: *mut ::libc::c_double,
                  neval: *mut ::libc::c_int,
                  ier: *mut ::libc::c_int,
                  limit: *mut ::libc::c_int,
                  lenw: *mut ::libc::c_int,
                  last: *mut ::libc::c_int,
                  iwork: *mut ::libc::c_int,
                  work: *mut ::libc::c_double)
                  -> ();
    pub fn vmmin(n: ::libc::c_int,
                 b: *mut ::libc::c_double,
                 Fmin: *mut ::libc::c_double,
                 _fn: optimfn,
                 gr: optimgr,
                 maxit: ::libc::c_int,
                 trace: ::libc::c_int,
                 mask: *mut ::libc::c_int,
                 abstol: ::libc::c_double,
                 reltol: ::libc::c_double,
                 nREPORT: ::libc::c_int,
                 ex: *mut ::libc::c_void,
                 fncount: *mut ::libc::c_int,
                 grcount: *mut ::libc::c_int,
                 fail: *mut ::libc::c_int)
                 -> ();
    pub fn nmmin(n: ::libc::c_int,
                 Bvec: *mut ::libc::c_double,
                 X: *mut ::libc::c_double,
                 Fmin: *mut ::libc::c_double,
                 _fn: optimfn,
                 fail: *mut ::libc::c_int,
                 abstol: ::libc::c_double,
                 intol: ::libc::c_double,
                 ex: *mut ::libc::c_void,
                 alpha: ::libc::c_double,
                 bet: ::libc::c_double,
                 gamm: ::libc::c_double,
                 trace: ::libc::c_int,
                 fncount: *mut ::libc::c_int,
                 maxit: ::libc::c_int)
                 -> ();
    pub fn cgmin(n: ::libc::c_int,
                 Bvec: *mut ::libc::c_double,
                 X: *mut ::libc::c_double,
                 Fmin: *mut ::libc::c_double,
                 _fn: optimfn,
                 gr: optimgr,
                 fail: *mut ::libc::c_int,
                 abstol: ::libc::c_double,
                 intol: ::libc::c_double,
                 ex: *mut ::libc::c_void,
                 _type: ::libc::c_int,
                 trace: ::libc::c_int,
                 fncount: *mut ::libc::c_int,
                 grcount: *mut ::libc::c_int,
                 maxit: ::libc::c_int)
                 -> ();
    pub fn lbfgsb(n: ::libc::c_int,
                  m: ::libc::c_int,
                  x: *mut ::libc::c_double,
                  l: *mut ::libc::c_double,
                  u: *mut ::libc::c_double,
                  nbd: *mut ::libc::c_int,
                  Fmin: *mut ::libc::c_double,
                  _fn: optimfn,
                  gr: optimgr,
                  fail: *mut ::libc::c_int,
                  ex: *mut ::libc::c_void,
                  factr: ::libc::c_double,
                  pgtol: ::libc::c_double,
                  fncount: *mut ::libc::c_int,
                  grcount: *mut ::libc::c_int,
                  maxit: ::libc::c_int,
                  msg: *mut ::libc::c_char,
                  trace: ::libc::c_int,
                  nREPORT: ::libc::c_int)
                  -> ();
    pub fn samin(n: ::libc::c_int,
                 pb: *mut ::libc::c_double,
                 yb: *mut ::libc::c_double,
                 _fn: optimfn,
                 maxit: ::libc::c_int,
                 tmax: ::libc::c_int,
                 ti: ::libc::c_double,
                 trace: ::libc::c_int,
                 ex: *mut ::libc::c_void)
                 -> ();
    pub fn findInterval(xt: *mut ::libc::c_double,
                        n: ::libc::c_int,
                        x: ::libc::c_double,
                        rightmost_closed: Rboolean,
                        all_inside: Rboolean,
                        ilo: ::libc::c_int,
                        mflag: *mut ::libc::c_int)
                        -> ::libc::c_int;
    pub fn dqrqty_(x: *mut ::libc::c_double,
                   n: *mut ::libc::c_int,
                   k: *mut ::libc::c_int,
                   qraux: *mut ::libc::c_double,
                   y: *mut ::libc::c_double,
                   ny: *mut ::libc::c_int,
                   qty: *mut ::libc::c_double)
                   -> ();
    pub fn dqrqy_(x: *mut ::libc::c_double,
                  n: *mut ::libc::c_int,
                  k: *mut ::libc::c_int,
                  qraux: *mut ::libc::c_double,
                  y: *mut ::libc::c_double,
                  ny: *mut ::libc::c_int,
                  qy: *mut ::libc::c_double)
                  -> ();
    pub fn dqrcf_(x: *mut ::libc::c_double,
                  n: *mut ::libc::c_int,
                  k: *mut ::libc::c_int,
                  qraux: *mut ::libc::c_double,
                  y: *mut ::libc::c_double,
                  ny: *mut ::libc::c_int,
                  b: *mut ::libc::c_double,
                  info: *mut ::libc::c_int)
                  -> ();
    pub fn dqrrsd_(x: *mut ::libc::c_double,
                   n: *mut ::libc::c_int,
                   k: *mut ::libc::c_int,
                   qraux: *mut ::libc::c_double,
                   y: *mut ::libc::c_double,
                   ny: *mut ::libc::c_int,
                   rsd: *mut ::libc::c_double)
                   -> ();
    pub fn dqrxb_(x: *mut ::libc::c_double,
                  n: *mut ::libc::c_int,
                  k: *mut ::libc::c_int,
                  qraux: *mut ::libc::c_double,
                  y: *mut ::libc::c_double,
                  ny: *mut ::libc::c_int,
                  xb: *mut ::libc::c_double)
                  -> ();
    pub fn R_pretty(lo: *mut ::libc::c_double,
                    up: *mut ::libc::c_double,
                    ndiv: *mut ::libc::c_int,
                    min_n: ::libc::c_int,
                    shrink_sml: ::libc::c_double,
                    high_u_fact: *mut ::libc::c_double,
                    eps_correction: ::libc::c_int,
                    return_bounds: ::libc::c_int)
                    -> ::libc::c_double;
    pub fn fdhess(n: ::libc::c_int,
                  x: *mut ::libc::c_double,
                  fval: ::libc::c_double,
                  fun: fcn_p,
                  state: *mut ::libc::c_void,
                  h: *mut ::libc::c_double,
                  nfd: ::libc::c_int,
                  step: *mut ::libc::c_double,
                  f: *mut ::libc::c_double,
                  ndigit: ::libc::c_int,
                  typx: *mut ::libc::c_double)
                  -> ();
    pub fn optif9(nr: ::libc::c_int,
                  n: ::libc::c_int,
                  x: *mut ::libc::c_double,
                  fcn: fcn_p,
                  d1fcn: fcn_p,
                  d2fcn: d2fcn_p,
                  state: *mut ::libc::c_void,
                  typsiz: *mut ::libc::c_double,
                  fscale: ::libc::c_double,
                  method: ::libc::c_int,
                  iexp: ::libc::c_int,
                  msg: *mut ::libc::c_int,
                  ndigit: ::libc::c_int,
                  itnlim: ::libc::c_int,
                  iagflg: ::libc::c_int,
                  iahflg: ::libc::c_int,
                  dlt: ::libc::c_double,
                  gradtl: ::libc::c_double,
                  stepmx: ::libc::c_double,
                  steptl: ::libc::c_double,
                  xpls: *mut ::libc::c_double,
                  fpls: *mut ::libc::c_double,
                  gpls: *mut ::libc::c_double,
                  itrmcd: *mut ::libc::c_int,
                  a: *mut ::libc::c_double,
                  wrk: *mut ::libc::c_double,
                  itncnt: *mut ::libc::c_int)
                  -> ();
    pub fn dqrdc2_(x: *mut ::libc::c_double,
                   ldx: *mut ::libc::c_int,
                   n: *mut ::libc::c_int,
                   p: *mut ::libc::c_int,
                   tol: *mut ::libc::c_double,
                   rank: *mut ::libc::c_int,
                   qraux: *mut ::libc::c_double,
                   pivot: *mut ::libc::c_int,
                   work: *mut ::libc::c_double)
                   -> ();
    pub fn dqrls_(x: *mut ::libc::c_double,
                  n: *mut ::libc::c_int,
                  p: *mut ::libc::c_int,
                  y: *mut ::libc::c_double,
                  ny: *mut ::libc::c_int,
                  tol: *mut ::libc::c_double,
                  b: *mut ::libc::c_double,
                  rsd: *mut ::libc::c_double,
                  qty: *mut ::libc::c_double,
                  k: *mut ::libc::c_int,
                  jpvt: *mut ::libc::c_int,
                  qraux: *mut ::libc::c_double,
                  work: *mut ::libc::c_double)
                  -> ();
}
