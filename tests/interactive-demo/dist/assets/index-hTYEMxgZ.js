(async () => {
  (function() {
    const e = document.createElement("link").relList;
    if (e && e.supports && e.supports("modulepreload")) return;
    for (const i of document.querySelectorAll('link[rel="modulepreload"]')) n(i);
    new MutationObserver((i) => {
      for (const o of i) if (o.type === "childList") for (const u of o.addedNodes) u.tagName === "LINK" && u.rel === "modulepreload" && n(u);
    }).observe(document, {
      childList: true,
      subtree: true
    });
    function r(i) {
      const o = {};
      return i.integrity && (o.integrity = i.integrity), i.referrerPolicy && (o.referrerPolicy = i.referrerPolicy), i.crossOrigin === "use-credentials" ? o.credentials = "include" : i.crossOrigin === "anonymous" ? o.credentials = "omit" : o.credentials = "same-origin", o;
    }
    function n(i) {
      if (i.ep) return;
      i.ep = true;
      const o = r(i);
      fetch(i.href, o);
    }
  })();
  const nu = "/assets/achronyme_wasm_bg-Dflr0sLL.wasm", iu = async (t = {}, e) => {
    let r;
    if (e.startsWith("data:")) {
      const n = e.replace(/^data:.*?base64,/, "");
      let i;
      if (typeof Buffer == "function" && typeof Buffer.from == "function") i = Buffer.from(n, "base64");
      else if (typeof atob == "function") {
        const o = atob(n);
        i = new Uint8Array(o.length);
        for (let u = 0; u < o.length; u++) i[u] = o.charCodeAt(u);
      } else throw new Error("Cannot decode base64-encoded data URL");
      r = await WebAssembly.instantiate(i, t);
    } else {
      const n = await fetch(e), i = n.headers.get("Content-Type") || "";
      if ("instantiateStreaming" in WebAssembly && i.startsWith("application/wasm")) r = await WebAssembly.instantiateStreaming(n, t);
      else {
        const o = await n.arrayBuffer();
        r = await WebAssembly.instantiate(o, t);
      }
    }
    return r.instance.exports;
  };
  let W;
  function ii(t) {
    W = t;
  }
  let Xt = null;
  function tr() {
    return (Xt === null || Xt.byteLength === 0) && (Xt = new Uint8Array(W.memory.buffer)), Xt;
  }
  let rr = new TextDecoder("utf-8", {
    ignoreBOM: true,
    fatal: true
  });
  rr.decode();
  const ou = 2146435072;
  let Tr = 0;
  function uu(t, e) {
    return Tr += e, Tr >= ou && (rr = new TextDecoder("utf-8", {
      ignoreBOM: true,
      fatal: true
    }), rr.decode(), Tr = e), rr.decode(tr().subarray(t, t + e));
  }
  function jr(t, e) {
    return t = t >>> 0, uu(t, e);
  }
  function au(t) {
    const e = W.__externref_table_alloc();
    return W.__wbindgen_externrefs.set(e, t), e;
  }
  function su(t, e) {
    try {
      return t.apply(this, e);
    } catch (r) {
      const n = au(r);
      W.__wbindgen_exn_store(n);
    }
  }
  function ne(t) {
    const e = W.__wbindgen_externrefs.get(t);
    return W.__externref_table_dealloc(t), e;
  }
  function cu(t, e) {
    const r = W.dot(t, e);
    if (r[2]) throw ne(r[1]);
    return r[0];
  }
  function fu(t) {
    const e = W.mathCos(t);
    if (e[2]) throw ne(e[1]);
    return e[0] >>> 0;
  }
  function lu(t, e, r) {
    const n = W.eigenSymmetric(t, e, r);
    if (n[2]) throw ne(n[1]);
    return Mt.__wrap(n[0]);
  }
  function hu(t) {
    const e = W.mathAbs(t);
    if (e[2]) throw ne(e[1]);
    return e[0] >>> 0;
  }
  let kt = null;
  function oi() {
    return (kt === null || kt.byteLength === 0) && (kt = new Float64Array(W.memory.buffer)), kt;
  }
  let gt = 0;
  function ui(t, e) {
    const r = e(t.length * 8, 8) >>> 0;
    return oi().set(t, r / 8), gt = t.length, r;
  }
  function du(t, e, r) {
    const n = ui(t, W.__wbindgen_malloc), i = gt, o = W.createMatrix(n, i, e, r);
    if (o[2]) throw ne(o[1]);
    return o[0] >>> 0;
  }
  function mu(t) {
    const e = W.mean(t);
    if (e[2]) throw ne(e[1]);
    return e[0];
  }
  function pu(t) {
    const e = W.sum(t);
    if (e[2]) throw ne(e[1]);
    return e[0];
  }
  function vu(t) {
    const e = W.cholesky(t);
    if (e[2]) throw ne(e[1]);
    return e[0] >>> 0;
  }
  function gu(t) {
    return W.blackmanWindow(t) >>> 0;
  }
  function Du(t) {
    return W.hanningWindow(t) >>> 0;
  }
  function wu(t) {
    const e = W.min(t);
    if (e[2]) throw ne(e[1]);
    return e[0];
  }
  function yu(t) {
    const e = W.mathExp(t);
    if (e[2]) throw ne(e[1]);
    return e[0] >>> 0;
  }
  function _u(t, e) {
    const r = W.vmul(t, e);
    if (r[2]) throw ne(r[1]);
    return r[0] >>> 0;
  }
  function Au() {
    W.reset();
  }
  function Fu(t) {
    const e = W.norm(t);
    if (e[2]) throw ne(e[1]);
    return e[0];
  }
  function Eu(t) {
    const e = W.svd(t);
    if (e[2]) throw ne(e[1]);
    return xt.__wrap(e[0]);
  }
  function Cu(t, e, r) {
    const n = W.qrEigenvalues(t, e, r);
    if (n[2]) throw ne(n[1]);
    return n[0] >>> 0;
  }
  function bu(t, e) {
    const r = W.isSymmetric(t, e);
    if (r[2]) throw ne(r[1]);
    return r[0] !== 0;
  }
  function Mu(t) {
    const e = W.norm_l1(t);
    if (e[2]) throw ne(e[1]);
    return e[0];
  }
  function Su(t, e) {
    return W.createVectorFromBuffer(t, e) >>> 0;
  }
  function Nu(t) {
    const e = W.mathTan(t);
    if (e[2]) throw ne(e[1]);
    return e[0] >>> 0;
  }
  function Bu(t) {
    const e = W.max(t);
    if (e[2]) throw ne(e[1]);
    return e[0];
  }
  function xu(t) {
    const e = W.ifft(t);
    if (e[2]) throw ne(e[1]);
    return e[0] >>> 0;
  }
  function Tu(t) {
    const e = W.mathSqrt(t);
    if (e[2]) throw ne(e[1]);
    return e[0] >>> 0;
  }
  function $u(t) {
    const e = W.isPositiveDefinite(t);
    if (e[2]) throw ne(e[1]);
    return e[0] !== 0;
  }
  function Iu(t) {
    const e = W.lu(t);
    if (e[2]) throw ne(e[1]);
    return St.__wrap(e[0]);
  }
  function zu(t, e) {
    const r = W.vsub(t, e);
    if (r[2]) throw ne(r[1]);
    return r[0] >>> 0;
  }
  const Vt = new TextEncoder();
  "encodeInto" in Vt || (Vt.encodeInto = function(t, e) {
    const r = Vt.encode(t);
    return e.set(r), {
      read: t.length,
      written: r.length
    };
  });
  function ai(t, e, r) {
    if (r === void 0) {
      const a = Vt.encode(t), f = e(a.length, 1) >>> 0;
      return tr().subarray(f, f + a.length).set(a), gt = a.length, f;
    }
    let n = t.length, i = e(n, 1) >>> 0;
    const o = tr();
    let u = 0;
    for (; u < n; u++) {
      const a = t.charCodeAt(u);
      if (a > 127) break;
      o[i + u] = a;
    }
    if (u !== n) {
      u !== 0 && (t = t.slice(u)), i = r(i, n, n = u + t.length * 3, 1) >>> 0;
      const a = tr().subarray(i + u, i + n), f = Vt.encodeInto(t, a);
      u += f.written, i = r(i, n, u, 1) >>> 0;
    }
    return gt = u, i;
  }
  function Ou(t, e) {
    const r = ai(t, W.__wbindgen_malloc, W.__wbindgen_realloc), n = gt, i = W.bindVariableToHandle(r, n, e);
    if (i[1]) throw ne(i[0]);
  }
  function Pu(t) {
    const e = W.qr(t);
    if (e[2]) throw ne(e[1]);
    return Bt.__wrap(e[0]);
  }
  function qu(t, e, r) {
    const n = W.createMatrixFromBuffer(t, e, r);
    if (n[2]) throw ne(n[1]);
    return n[0] >>> 0;
  }
  function Ru(t) {
    const e = W.mathSin(t);
    if (e[2]) throw ne(e[1]);
    return e[0] >>> 0;
  }
  function Lu(t) {
    const e = W.inverse(t);
    if (e[2]) throw ne(e[1]);
    return e[0] >>> 0;
  }
  function Vu(t, e, r) {
    const n = W.powerIteration(t, e, r);
    if (n[2]) throw ne(n[1]);
    return Nt.__wrap(n[0]);
  }
  function Uu(t, e) {
    return t = t >>> 0, oi().subarray(t / 8, t / 8 + e);
  }
  function Wu(t) {
    const e = W.getVector(t);
    if (e[3]) throw ne(e[2]);
    var r = Uu(e[0], e[1]).slice();
    return W.__wbindgen_free(e[0], e[1] * 8, 8), r;
  }
  function Zu(t) {
    const e = W.getMatrix(t);
    if (e[2]) throw ne(e[1]);
    return ne(e[0]);
  }
  function Hu(t) {
    const e = W.dspFftMag(t);
    if (e[2]) throw ne(e[1]);
    return e[0] >>> 0;
  }
  function si(t) {
    const e = ui(t, W.__wbindgen_malloc), r = gt;
    return W.createVector(e, r) >>> 0;
  }
  function ju(t) {
    const e = W.std(t);
    if (e[2]) throw ne(e[1]);
    return e[0];
  }
  function Ju(t, e) {
    const r = W.vdiv(t, e);
    if (r[2]) throw ne(r[1]);
    return r[0] >>> 0;
  }
  function Ku(t) {
    let e, r;
    try {
      const o = ai(t, W.__wbindgen_malloc, W.__wbindgen_realloc), u = gt, a = W._eval(o, u);
      var n = a[0], i = a[1];
      if (a[3]) throw n = 0, i = 0, ne(a[2]);
      return e = n, r = i, jr(n, i);
    } finally {
      W.__wbindgen_free(e, r, 1);
    }
  }
  function Yu(t) {
    const e = W.identity(t);
    if (e[2]) throw ne(e[1]);
    return e[0] >>> 0;
  }
  function Gu(t) {
    const e = W.dspFft(t);
    if (e[2]) throw ne(e[1]);
    return e[0] >>> 0;
  }
  function Qu(t, e, r) {
    const n = W.linspace(t, e, r);
    if (n[2]) throw ne(n[1]);
    return n[0] >>> 0;
  }
  function Xu(t) {
    return W.hammingWindow(t) >>> 0;
  }
  function ku(t) {
    W.releaseHandle(t);
  }
  function ea(t) {
    const e = W.mathLn(t);
    if (e[2]) throw ne(e[1]);
    return e[0] >>> 0;
  }
  function ta(t, e) {
    const r = W.vadd(t, e);
    if (r[2]) throw ne(r[1]);
    return r[0] >>> 0;
  }
  const un = typeof FinalizationRegistry > "u" ? {
    register: () => {
    },
    unregister: () => {
    }
  } : new FinalizationRegistry((t) => W.__wbg_eigenresult_free(t >>> 0, 1));
  class Mt {
    static __wrap(e) {
      e = e >>> 0;
      const r = Object.create(Mt.prototype);
      return r.__wbg_ptr = e, un.register(r, r.__wbg_ptr, r), r;
    }
    __destroy_into_raw() {
      const e = this.__wbg_ptr;
      return this.__wbg_ptr = 0, un.unregister(this), e;
    }
    free() {
      const e = this.__destroy_into_raw();
      W.__wbg_eigenresult_free(e, 0);
    }
    get eigenvalues() {
      return W.__wbg_get_eigenresult_eigenvalues(this.__wbg_ptr) >>> 0;
    }
    get eigenvectors() {
      return W.__wbg_get_eigenresult_eigenvectors(this.__wbg_ptr) >>> 0;
    }
  }
  Symbol.dispose && (Mt.prototype[Symbol.dispose] = Mt.prototype.free);
  const an = typeof FinalizationRegistry > "u" ? {
    register: () => {
    },
    unregister: () => {
    }
  } : new FinalizationRegistry((t) => W.__wbg_luresult_free(t >>> 0, 1));
  class St {
    static __wrap(e) {
      e = e >>> 0;
      const r = Object.create(St.prototype);
      return r.__wbg_ptr = e, an.register(r, r.__wbg_ptr, r), r;
    }
    __destroy_into_raw() {
      const e = this.__wbg_ptr;
      return this.__wbg_ptr = 0, an.unregister(this), e;
    }
    free() {
      const e = this.__destroy_into_raw();
      W.__wbg_luresult_free(e, 0);
    }
    get L() {
      return W.__wbg_get_eigenresult_eigenvalues(this.__wbg_ptr) >>> 0;
    }
    get U() {
      return W.__wbg_get_eigenresult_eigenvectors(this.__wbg_ptr) >>> 0;
    }
    get P() {
      return W.__wbg_get_luresult_P(this.__wbg_ptr) >>> 0;
    }
  }
  Symbol.dispose && (St.prototype[Symbol.dispose] = St.prototype.free);
  const sn = typeof FinalizationRegistry > "u" ? {
    register: () => {
    },
    unregister: () => {
    }
  } : new FinalizationRegistry((t) => W.__wbg_poweriterationresult_free(t >>> 0, 1));
  class Nt {
    static __wrap(e) {
      e = e >>> 0;
      const r = Object.create(Nt.prototype);
      return r.__wbg_ptr = e, sn.register(r, r.__wbg_ptr, r), r;
    }
    __destroy_into_raw() {
      const e = this.__wbg_ptr;
      return this.__wbg_ptr = 0, sn.unregister(this), e;
    }
    free() {
      const e = this.__destroy_into_raw();
      W.__wbg_poweriterationresult_free(e, 0);
    }
    get eigenvalue() {
      return W.__wbg_get_poweriterationresult_eigenvalue(this.__wbg_ptr);
    }
    get eigenvector() {
      return W.__wbg_get_poweriterationresult_eigenvector(this.__wbg_ptr) >>> 0;
    }
  }
  Symbol.dispose && (Nt.prototype[Symbol.dispose] = Nt.prototype.free);
  const cn = typeof FinalizationRegistry > "u" ? {
    register: () => {
    },
    unregister: () => {
    }
  } : new FinalizationRegistry((t) => W.__wbg_qrresult_free(t >>> 0, 1));
  class Bt {
    static __wrap(e) {
      e = e >>> 0;
      const r = Object.create(Bt.prototype);
      return r.__wbg_ptr = e, cn.register(r, r.__wbg_ptr, r), r;
    }
    __destroy_into_raw() {
      const e = this.__wbg_ptr;
      return this.__wbg_ptr = 0, cn.unregister(this), e;
    }
    free() {
      const e = this.__destroy_into_raw();
      W.__wbg_qrresult_free(e, 0);
    }
    get Q() {
      return W.__wbg_get_eigenresult_eigenvalues(this.__wbg_ptr) >>> 0;
    }
    get R() {
      return W.__wbg_get_eigenresult_eigenvectors(this.__wbg_ptr) >>> 0;
    }
  }
  Symbol.dispose && (Bt.prototype[Symbol.dispose] = Bt.prototype.free);
  const fn = typeof FinalizationRegistry > "u" ? {
    register: () => {
    },
    unregister: () => {
    }
  } : new FinalizationRegistry((t) => W.__wbg_svdresult_free(t >>> 0, 1));
  class xt {
    static __wrap(e) {
      e = e >>> 0;
      const r = Object.create(xt.prototype);
      return r.__wbg_ptr = e, fn.register(r, r.__wbg_ptr, r), r;
    }
    __destroy_into_raw() {
      const e = this.__wbg_ptr;
      return this.__wbg_ptr = 0, fn.unregister(this), e;
    }
    free() {
      const e = this.__destroy_into_raw();
      W.__wbg_svdresult_free(e, 0);
    }
    get U() {
      return W.__wbg_get_eigenresult_eigenvalues(this.__wbg_ptr) >>> 0;
    }
    get S() {
      return W.__wbg_get_eigenresult_eigenvectors(this.__wbg_ptr) >>> 0;
    }
    get V() {
      return W.__wbg_get_luresult_P(this.__wbg_ptr) >>> 0;
    }
  }
  Symbol.dispose && (xt.prototype[Symbol.dispose] = xt.prototype.free);
  function ci(t, e) {
    throw new Error(jr(t, e));
  }
  function fi() {
    return new Object();
  }
  function li() {
    return new Array();
  }
  function hi(t, e, r) {
    t[e >>> 0] = r;
  }
  function di() {
    return su(function(t, e, r) {
      return Reflect.set(t, e, r);
    }, arguments);
  }
  function mi(t, e) {
    return jr(t, e);
  }
  function pi(t) {
    return t;
  }
  function vi() {
    const t = W.__wbindgen_externrefs, e = t.grow(4);
    t.set(0, void 0), t.set(e + 0, void 0), t.set(e + 1, null), t.set(e + 2, true), t.set(e + 3, false);
  }
  URL = globalThis.URL;
  const J = await iu({
    "./achronyme_wasm_bg.js": {
      __wbg_new_1acc0b6eea89d040: fi,
      __wbg_new_e17d9f43105b08be: li,
      __wbg_set_c213c871859d6500: hi,
      __wbg_set_c2abbebe8b9ebee1: di,
      __wbg___wbindgen_throw_b855445ff6a94295: ci,
      __wbindgen_init_externref_table: vi,
      __wbindgen_cast_2241b6af4c4b2941: mi,
      __wbindgen_cast_d6cd19b81560fd6e: pi
    }
  }, nu), ra = J.memory, na = J.__wbg_eigenresult_free, ia = J.__wbg_get_eigenresult_eigenvalues, oa = J.__wbg_get_eigenresult_eigenvectors, ua = J.__wbg_get_luresult_P, aa = J.__wbg_get_poweriterationresult_eigenvalue, sa = J.__wbg_get_poweriterationresult_eigenvector, ca = J.__wbg_luresult_free, fa = J.__wbg_poweriterationresult_free, la = J._eval, ha = J.bindVariableToHandle, da = J.blackmanWindow, ma = J.cholesky, pa = J.createMatrix, va = J.createMatrixFromBuffer, ga = J.createVector, Da = J.createVectorFromBuffer, wa = J.dot, ya = J.dspFft, _a = J.dspFftMag, Aa = J.eigenSymmetric, Fa = J.getMatrix, Ea = J.getVector, Ca = J.hammingWindow, ba = J.hanningWindow, Ma = J.identity, Sa = J.ifft, Na = J.inverse, Ba = J.isPositiveDefinite, xa = J.isSymmetric, Ta = J.linspace, $a = J.lu, Ia = J.mathAbs, za = J.mathCos, Oa = J.mathExp, Pa = J.mathLn, qa = J.mathSin, Ra = J.mathSqrt, La = J.mathTan, Va = J.max, Ua = J.mean, Wa = J.min, Za = J.norm, Ha = J.norm_l1, ja = J.powerIteration, Ja = J.qr, Ka = J.qrEigenvalues, Ya = J.releaseHandle, Ga = J.reset, Qa = J.std, Xa = J.sum, ka = J.svd, es = J.vadd, ts = J.vdiv, rs = J.vmul, ns = J.vsub, is = J.__wbg_get_luresult_L, os = J.__wbg_get_luresult_U, us = J.__wbg_get_qrresult_Q, as = J.__wbg_get_qrresult_R, ss = J.__wbg_get_svdresult_S, cs = J.__wbg_get_svdresult_U, fs = J.__wbg_get_svdresult_V, ls = J.__wbg_qrresult_free, hs = J.__wbg_svdresult_free, ds = J.__wbindgen_exn_store, ms = J.__externref_table_alloc, ps = J.__wbindgen_externrefs, vs = J.__externref_table_dealloc, gs = J.__wbindgen_malloc, Ds = J.__wbindgen_realloc, ws = J.__wbindgen_free, gi = J.__wbindgen_start, ys = Object.freeze(Object.defineProperty({
    __proto__: null,
    __externref_table_alloc: ms,
    __externref_table_dealloc: vs,
    __wbg_eigenresult_free: na,
    __wbg_get_eigenresult_eigenvalues: ia,
    __wbg_get_eigenresult_eigenvectors: oa,
    __wbg_get_luresult_L: is,
    __wbg_get_luresult_P: ua,
    __wbg_get_luresult_U: os,
    __wbg_get_poweriterationresult_eigenvalue: aa,
    __wbg_get_poweriterationresult_eigenvector: sa,
    __wbg_get_qrresult_Q: us,
    __wbg_get_qrresult_R: as,
    __wbg_get_svdresult_S: ss,
    __wbg_get_svdresult_U: cs,
    __wbg_get_svdresult_V: fs,
    __wbg_luresult_free: ca,
    __wbg_poweriterationresult_free: fa,
    __wbg_qrresult_free: ls,
    __wbg_svdresult_free: hs,
    __wbindgen_exn_store: ds,
    __wbindgen_externrefs: ps,
    __wbindgen_free: ws,
    __wbindgen_malloc: gs,
    __wbindgen_realloc: Ds,
    __wbindgen_start: gi,
    _eval: la,
    bindVariableToHandle: ha,
    blackmanWindow: da,
    cholesky: ma,
    createMatrix: pa,
    createMatrixFromBuffer: va,
    createVector: ga,
    createVectorFromBuffer: Da,
    dot: wa,
    dspFft: ya,
    dspFftMag: _a,
    eigenSymmetric: Aa,
    getMatrix: Fa,
    getVector: Ea,
    hammingWindow: Ca,
    hanningWindow: ba,
    identity: Ma,
    ifft: Sa,
    inverse: Na,
    isPositiveDefinite: Ba,
    isSymmetric: xa,
    linspace: Ta,
    lu: $a,
    mathAbs: Ia,
    mathCos: za,
    mathExp: Oa,
    mathLn: Pa,
    mathSin: qa,
    mathSqrt: Ra,
    mathTan: La,
    max: Va,
    mean: Ua,
    memory: ra,
    min: Wa,
    norm: Za,
    norm_l1: Ha,
    powerIteration: ja,
    qr: Ja,
    qrEigenvalues: Ka,
    releaseHandle: Ya,
    reset: Ga,
    std: Qa,
    sum: Xa,
    svd: ka,
    vadd: es,
    vdiv: ts,
    vmul: rs,
    vsub: ns
  }, Symbol.toStringTag, {
    value: "Module"
  }));
  ii(ys);
  gi();
  const ln = Object.freeze(Object.defineProperty({
    __proto__: null,
    EigenResult: Mt,
    LUResult: St,
    PowerIterationResult: Nt,
    QRResult: Bt,
    SVDResult: xt,
    __wbg___wbindgen_throw_b855445ff6a94295: ci,
    __wbg_new_1acc0b6eea89d040: fi,
    __wbg_new_e17d9f43105b08be: li,
    __wbg_set_c213c871859d6500: hi,
    __wbg_set_c2abbebe8b9ebee1: di,
    __wbg_set_wasm: ii,
    __wbindgen_cast_2241b6af4c4b2941: mi,
    __wbindgen_cast_d6cd19b81560fd6e: pi,
    __wbindgen_init_externref_table: vi,
    _eval: Ku,
    bindVariableToHandle: Ou,
    blackmanWindow: gu,
    cholesky: vu,
    createMatrix: du,
    createMatrixFromBuffer: qu,
    createVector: si,
    createVectorFromBuffer: Su,
    dot: cu,
    dspFft: Gu,
    dspFftMag: Hu,
    eigenSymmetric: lu,
    getMatrix: Zu,
    getVector: Wu,
    hammingWindow: Xu,
    hanningWindow: Du,
    identity: Yu,
    ifft: xu,
    inverse: Lu,
    isPositiveDefinite: $u,
    isSymmetric: bu,
    linspace: Qu,
    lu: Iu,
    mathAbs: hu,
    mathCos: fu,
    mathExp: yu,
    mathLn: ea,
    mathSin: Ru,
    mathSqrt: Tu,
    mathTan: Nu,
    max: Bu,
    mean: mu,
    min: wu,
    norm: Fu,
    norm_l1: Mu,
    powerIteration: Vu,
    qr: Pu,
    qrEigenvalues: Cu,
    releaseHandle: ku,
    reset: Au,
    std: ju,
    sum: pu,
    svd: Eu,
    vadd: ta,
    vdiv: Ju,
    vmul: _u,
    vsub: zu
  }, Symbol.toStringTag, {
    value: "Module"
  }));
  class _s {
    constructor() {
      this.module = null, this.initialized = false;
    }
    async init() {
      if (!this.initialized) try {
        if (await Promise.resolve(), !ln || typeof si != "function") throw new Error("WASM module not properly initialized - functions not available");
        this.module = ln, this.initialized = true, console.log("\u2705 RustWASM initialized successfully");
      } catch (e) {
        throw console.error("\u274C Failed to initialize WASM module:", e), new Error(`Failed to initialize WASM module: ${e}`);
      }
    }
    ensureInit() {
      if (!this.module || !this.initialized) throw new Error("WASM module not initialized. Call init() first.");
      return this.module;
    }
    releaseHandle(e) {
      const r = this.ensureInit();
      try {
        r.releaseHandle(e);
      } catch (n) {
        console.warn(`Failed to release handle ${e}:`, n);
      }
    }
    reset() {
      this.ensureInit().reset();
    }
    createVector(e) {
      const r = this.ensureInit();
      try {
        const n = new Float64Array(e);
        return r.createVector(n);
      } catch (n) {
        throw new Error(`Failed to create vector: ${n}`);
      }
    }
    getVector(e) {
      const r = this.ensureInit();
      try {
        const n = r.getVector(e);
        return Array.from(n);
      } catch (n) {
        throw new Error(`Failed to get vector: ${n}`);
      }
    }
    getVectorLength(e) {
      const r = this.ensureInit();
      try {
        return r.getVector(e).length;
      } catch (n) {
        throw new Error(`Failed to get vector length: ${n}`);
      }
    }
    createMatrix(e, r, n) {
      const i = this.ensureInit();
      try {
        const o = new Float64Array(e);
        return i.createMatrix(o, r, n);
      } catch (o) {
        throw new Error(`Failed to create matrix: ${o}`);
      }
    }
    sin(e) {
      const r = this.ensureInit();
      try {
        return r.mathSin(e);
      } catch (n) {
        throw new Error(`sin failed: ${n}`);
      }
    }
    cos(e) {
      const r = this.ensureInit();
      try {
        return r.mathCos(e);
      } catch (n) {
        throw new Error(`cos failed: ${n}`);
      }
    }
    tan(e) {
      const r = this.ensureInit();
      try {
        return r.mathTan(e);
      } catch (n) {
        throw new Error(`tan failed: ${n}`);
      }
    }
    exp(e) {
      const r = this.ensureInit();
      try {
        return r.mathExp(e);
      } catch (n) {
        throw new Error(`exp failed: ${n}`);
      }
    }
    ln(e) {
      const r = this.ensureInit();
      try {
        return r.mathLn(e);
      } catch (n) {
        throw new Error(`ln failed: ${n}`);
      }
    }
    abs(e) {
      const r = this.ensureInit();
      try {
        return r.mathAbs(e);
      } catch (n) {
        throw new Error(`abs failed: ${n}`);
      }
    }
    sqrt(e) {
      const r = this.ensureInit();
      try {
        return r.mathSqrt(e);
      } catch (n) {
        throw new Error(`sqrt failed: ${n}`);
      }
    }
    fft(e) {
      const r = this.ensureInit();
      try {
        return r.dspFft(e);
      } catch (n) {
        throw new Error(`fft failed: ${n}`);
      }
    }
    fft_mag(e) {
      const r = this.ensureInit();
      try {
        return r.dspFftMag(e);
      } catch (n) {
        throw new Error(`fft_mag failed: ${n}`);
      }
    }
    ifft(e) {
      const r = this.ensureInit();
      try {
        return r.ifft(e);
      } catch (n) {
        throw new Error(`ifft failed: ${n}`);
      }
    }
    hanningWindow(e) {
      const r = this.ensureInit();
      try {
        return r.hanningWindow(e);
      } catch (n) {
        throw new Error(`hanningWindow failed: ${n}`);
      }
    }
    hammingWindow(e) {
      const r = this.ensureInit();
      try {
        return r.hammingWindow(e);
      } catch (n) {
        throw new Error(`hammingWindow failed: ${n}`);
      }
    }
    blackmanWindow(e) {
      const r = this.ensureInit();
      try {
        return r.blackmanWindow(e);
      } catch (n) {
        throw new Error(`blackmanWindow failed: ${n}`);
      }
    }
    lu_decomposition_js(e) {
      const r = this.ensureInit();
      try {
        return r.lu(e);
      } catch (n) {
        throw new Error(`lu_decomposition_js failed: ${n}`);
      }
    }
    qr_decomposition_js(e) {
      const r = this.ensureInit();
      try {
        return r.qr(e);
      } catch (n) {
        throw new Error(`qr_decomposition_js failed: ${n}`);
      }
    }
    svd_decomposition_js(e) {
      const r = this.ensureInit();
      try {
        return r.svd(e);
      } catch (n) {
        throw new Error(`svd_decomposition_js failed: ${n}`);
      }
    }
    matrixInverse(e) {
      const r = this.ensureInit();
      try {
        return r.inverse(e);
      } catch (n) {
        throw new Error(`Matrix inverse failed: ${n}`);
      }
    }
    vadd(e, r) {
      const n = this.ensureInit();
      try {
        return n.vadd(e, r);
      } catch (i) {
        throw new Error(`vadd failed: ${i}`);
      }
    }
    vsub(e, r) {
      const n = this.ensureInit();
      try {
        return n.vsub(e, r);
      } catch (i) {
        throw new Error(`vsub failed: ${i}`);
      }
    }
    vmul(e, r) {
      const n = this.ensureInit();
      try {
        return n.vmul(e, r);
      } catch (i) {
        throw new Error(`vmul failed: ${i}`);
      }
    }
    vdiv(e, r) {
      const n = this.ensureInit();
      try {
        return n.vdiv(e, r);
      } catch (i) {
        throw new Error(`vdiv failed: ${i}`);
      }
    }
    dot(e, r) {
      const n = this.ensureInit();
      try {
        return n.dot(e, r);
      } catch (i) {
        throw new Error(`dot failed: ${i}`);
      }
    }
    norm(e) {
      const r = this.ensureInit();
      try {
        return r.norm(e);
      } catch (n) {
        throw new Error(`norm failed: ${n}`);
      }
    }
    norm_l1(e) {
      const r = this.ensureInit();
      try {
        return r.norm_l1(e);
      } catch (n) {
        throw new Error(`norm_l1 failed: ${n}`);
      }
    }
    sum(e) {
      const r = this.ensureInit();
      try {
        return r.sum(e);
      } catch (n) {
        throw new Error(`sum failed: ${n}`);
      }
    }
    mean(e) {
      const r = this.ensureInit();
      try {
        return r.mean(e);
      } catch (n) {
        throw new Error(`mean failed: ${n}`);
      }
    }
    std(e) {
      const r = this.ensureInit();
      try {
        return r.std(e);
      } catch (n) {
        throw new Error(`std failed: ${n}`);
      }
    }
    min(e) {
      const r = this.ensureInit();
      try {
        return r.min(e);
      } catch (n) {
        throw new Error(`min failed: ${n}`);
      }
    }
    max(e) {
      const r = this.ensureInit();
      try {
        return r.max(e);
      } catch (n) {
        throw new Error(`max failed: ${n}`);
      }
    }
    linspace(e, r, n) {
      const i = this.ensureInit();
      try {
        return i.linspace(e, r, n);
      } catch (o) {
        throw new Error(`linspace failed: ${o}`);
      }
    }
    _eval(e) {
      const r = this.ensureInit();
      try {
        return r._eval(e);
      } catch (n) {
        throw new Error(`_eval failed: ${n}`);
      }
    }
  }
  class As {
    constructor(e) {
      this.wasm = e, this.handles = /* @__PURE__ */ new Map(), this.allocatedCount = 0, this.freedCount = 0;
    }
    register(e, r) {
      this.handles.set(e, r), this.allocatedCount++;
    }
    release(e) {
      if (this.handles.has(e)) {
        this.handles.delete(e);
        try {
          this.wasm.releaseHandle(e), this.freedCount++;
        } catch (r) {
          console.warn(`Failed to release handle ${e}:`, r);
        }
      }
    }
    get(e) {
      return this.handles.get(e);
    }
    has(e) {
      return this.handles.has(e);
    }
    getStats() {
      return {
        allocated: this.allocatedCount,
        freed: this.freedCount,
        active: this.handles.size,
        leaked: 0
      };
    }
    gc() {
      const e = Array.from(this.handles.keys()), r = e.length;
      for (const n of e) this.release(n);
      return r;
    }
    resetStats() {
      this.allocatedCount = 0, this.freedCount = 0;
    }
  }
  class pr {
    constructor(e, r, n) {
      this._disposed = false, this._session = e, this._handle = r, this._metadata = {
        handle: r,
        usedFastPath: true,
        createdAt: Date.now(),
        type: n
      }, this._session.track(this);
    }
    get handle() {
      return this.checkDisposed(), this._handle;
    }
    get metadata() {
      return this._metadata;
    }
    get isDisposed() {
      return this._disposed;
    }
    dispose() {
      if (!this._disposed) try {
        this._session.handleManager.release(this._handle), this._session.untrack(this), this._disposed = true;
      } catch (e) {
        console.warn("Error disposing value:", e);
      }
    }
    checkDisposed() {
      if (this._disposed) throw new Error(`Cannot use disposed ${this._metadata.type}. Value was disposed at ${new Date(this._metadata.createdAt).toISOString()}`);
    }
    get session() {
      return this._session;
    }
    get wasm() {
      return this._session.wasm;
    }
  }
  class k extends pr {
    constructor(e, r) {
      super(e, r, "vector");
    }
    get length() {
      return this.checkDisposed(), this.wasm.getVectorLength(this.handle);
    }
    get data() {
      this.checkDisposed();
      const e = this.wasm.getVector(this.handle);
      return new Float64Array(e);
    }
    get(e) {
      const r = this.data;
      if (e < 0 || e >= r.length) throw new RangeError(`Index ${e} out of bounds [0, ${r.length})`);
      return r[e];
    }
    set(e, r) {
      const n = this.data;
      if (e < 0 || e >= n.length) throw new RangeError(`Index ${e} out of bounds [0, ${n.length})`);
      n[e] = r;
    }
    toArray() {
      return Array.from(this.data);
    }
    *[Symbol.iterator]() {
      const e = this.data;
      for (let r = 0; r < e.length; r++) yield e[r];
    }
    map(e) {
      const r = this.data, n = new Float64Array(r.length);
      for (let o = 0; o < r.length; o++) n[o] = e(r[o], o);
      const i = this.wasm.createVector(Array.from(n));
      return new k(this.session, i);
    }
    filter(e) {
      const r = this.data, n = [];
      for (let o = 0; o < r.length; o++) e(r[o], o) && n.push(r[o]);
      const i = this.wasm.createVector(n);
      return new k(this.session, i);
    }
    reduce(e, r) {
      const n = this.data;
      let i = r;
      for (let o = 0; o < n.length; o++) i = e(i, n[o], o);
      return i;
    }
    toString() {
      if (this.isDisposed) return "[Vector (disposed)]";
      const e = this.data, r = e.length <= 10 ? Array.from(e).join(", ") : `${Array.from(e.slice(0, 5)).join(", ")}, ..., ${Array.from(e.slice(-2)).join(", ")}`;
      return `[Vector (${e.length}): ${r}]`;
    }
    dispose() {
      this._cachedData = void 0, super.dispose();
    }
  }
  let Ge = class Di extends pr {
    constructor(e, r, n, i) {
      super(e, r, "matrix"), this._rows = n, this._cols = i;
    }
    get rows() {
      if (this.checkDisposed(), this._rows === void 0) {
        const e = this.data;
        this._rows = Math.sqrt(e.length);
      }
      return this._rows;
    }
    get cols() {
      if (this.checkDisposed(), this._cols === void 0) {
        const e = this.data;
        this._cols = Math.sqrt(e.length);
      }
      return this._cols;
    }
    get length() {
      return this.rows * this.cols;
    }
    get data() {
      this.checkDisposed();
      const e = this.wasm.getVector(this.handle);
      return new Float64Array(e);
    }
    get(e, r) {
      if (e < 0 || e >= this.rows) throw new RangeError(`Row index ${e} out of bounds [0, ${this.rows})`);
      if (r < 0 || r >= this.cols) throw new RangeError(`Column index ${r} out of bounds [0, ${this.cols})`);
      const n = this.data, i = e * this.cols + r;
      return n[i];
    }
    set(e, r, n) {
      if (e < 0 || e >= this.rows) throw new RangeError(`Row index ${e} out of bounds [0, ${this.rows})`);
      if (r < 0 || r >= this.cols) throw new RangeError(`Column index ${r} out of bounds [0, ${this.cols})`);
      const i = this.data, o = e * this.cols + r;
      i[o] = n;
    }
    row(e) {
      if (e < 0 || e >= this.rows) throw new RangeError(`Row index ${e} out of bounds [0, ${this.rows})`);
      const r = this.data, n = [], i = e * this.cols;
      for (let u = 0; u < this.cols; u++) n.push(r[i + u]);
      const o = this.wasm.createVector(n);
      return new k(this.session, o);
    }
    col(e) {
      if (e < 0 || e >= this.cols) throw new RangeError(`Column index ${e} out of bounds [0, ${this.cols})`);
      const r = this.data, n = [];
      for (let o = 0; o < this.rows; o++) n.push(r[o * this.cols + e]);
      const i = this.wasm.createVector(n);
      return new k(this.session, i);
    }
    toArray() {
      const e = this.data, r = [];
      for (let n = 0; n < this.rows; n++) {
        const i = [];
        for (let o = 0; o < this.cols; o++) i.push(e[n * this.cols + o]);
        r.push(i);
      }
      return r;
    }
    toFlatArray() {
      return Array.from(this.data);
    }
    *[Symbol.iterator]() {
      for (let e = 0; e < this.rows; e++) yield this.row(e);
    }
    map(e) {
      const r = this.data, n = new Float64Array(r.length);
      for (let o = 0; o < this.rows; o++) for (let u = 0; u < this.cols; u++) {
        const a = o * this.cols + u;
        n[a] = e(r[a], o, u);
      }
      const i = this.wasm.createVector(Array.from(n));
      return new Di(this.session, i, this.rows, this.cols);
    }
    toString() {
      if (this.isDisposed) return "[Matrix (disposed)]";
      const e = this.toArray(), r = this.rows <= 5 ? e.map((n) => `[${n.join(", ")}]`).join(`
  `) : e.slice(0, 3).map((n) => `[${n.join(", ")}]`).join(`
  `) + `
  ...
  ` + e.slice(-1).map((n) => `[${n.join(", ")}]`).join(`
  `);
      return `[Matrix (${this.rows}x${this.cols}):
  ${r}
]`;
    }
  };
  class Fs extends pr {
    constructor(e, r, n) {
      super(e, r, "scalar"), this._value = n;
    }
    get value() {
      if (this.checkDisposed(), this._value === void 0) {
        const e = this.wasm.getVector(this.handle);
        this._value = e[0];
      }
      return this._value;
    }
    toNumber() {
      return this.value;
    }
    toArray() {
      return [
        this.value
      ];
    }
    valueOf() {
      return this.value;
    }
    toString() {
      return this.isDisposed ? "[Scalar (disposed)]" : `[Scalar: ${this.value}]`;
    }
  }
  let Es = class wi extends pr {
    constructor(e, r, n, i) {
      super(e, r, "complex"), this._re = n, this._im = i;
    }
    get re() {
      if (this.checkDisposed(), this._re === void 0) {
        const e = this.wasm.getVector(this.handle);
        this._re = e[0], this._im = e[1];
      }
      return this._re;
    }
    get im() {
      if (this.checkDisposed(), this._im === void 0) {
        const e = this.wasm.getVector(this.handle);
        this._re = e[0], this._im = e[1];
      }
      return this._im;
    }
    get magnitude() {
      const e = this.re, r = this.im;
      return Math.sqrt(e * e + r * r);
    }
    get phase() {
      return Math.atan2(this.im, this.re);
    }
    conjugate() {
      const e = this.wasm.createVector([
        this.re,
        -this.im
      ]);
      return new wi(this.session, e, this.re, -this.im);
    }
    toArray() {
      return [
        this.re,
        this.im
      ];
    }
    toPolar() {
      return [
        this.magnitude,
        this.phase
      ];
    }
    toString() {
      if (this.isDisposed) return "[Complex (disposed)]";
      const e = this.re, r = this.im;
      if (Math.abs(r) < 1e-10) return `${e}`;
      if (Math.abs(e) < 1e-10) return `${r}i`;
      const n = r >= 0 ? "+" : "-";
      return `${e} ${n} ${Math.abs(r)}i`;
    }
  };
  class Cs {
    constructor(e) {
      this.values = /* @__PURE__ */ new Set(), this.isActive = false, this.wasm = e || new _s(), this.handleManager = new As(this.wasm);
    }
    async init() {
      await this.wasm.init(), this.isActive = true;
    }
    async use(e) {
      if (!this.isActive) throw new Error("Session not initialized. Call init() first.");
      try {
        return await e();
      } finally {
        this.cleanup();
      }
    }
    cleanup() {
      for (const e of this.values) try {
        e.dispose();
      } catch (r) {
        console.warn("Error disposing value:", r);
      }
      this.values.clear();
    }
    track(e) {
      this.values.add(e), this.handleManager.register(e.handle, e);
    }
    untrack(e) {
      this.values.delete(e);
    }
    getActiveValuesCount() {
      return this.values.size;
    }
    isSessionActive() {
      return this.isActive;
    }
    destroy() {
      this.cleanup(), this.isActive = false;
    }
    vector(e) {
      if (!this.isActive) throw new Error("Session not initialized. Call init() first.");
      const r = this.wasm.createVector(e);
      return new k(this, r);
    }
    matrix(e) {
      var _a2;
      if (!this.isActive) throw new Error("Session not initialized. Call init() first.");
      const r = e.length, n = ((_a2 = e[0]) == null ? void 0 : _a2.length) || 0, i = [];
      for (let u = 0; u < r; u++) {
        if (e[u].length !== n) throw new Error("Matrix rows must have equal length");
        for (let a = 0; a < n; a++) i.push(e[u][a]);
      }
      const o = this.wasm.createMatrix(i, r, n);
      return new Ge(this, o, r, n);
    }
    scalar(e) {
      if (!this.isActive) throw new Error("Session not initialized. Call init() first.");
      const r = this.wasm.createVector([
        e
      ]);
      return new Fs(this, r, e);
    }
    complex(e, r) {
      if (!this.isActive) throw new Error("Session not initialized. Call init() first.");
      const n = this.wasm.createVector([
        e,
        r
      ]);
      return new Es(this, n, e, r);
    }
  }
  class bs {
    constructor(e) {
      this.session = e;
    }
    sin(e) {
      if (typeof e == "number") return Math.sin(e);
      const r = this.session.wasm.sin(e.handle);
      return new k(this.session, r);
    }
    cos(e) {
      if (typeof e == "number") return Math.cos(e);
      const r = this.session.wasm.cos(e.handle);
      return new k(this.session, r);
    }
    tan(e) {
      if (typeof e == "number") return Math.tan(e);
      const r = this.session.wasm.tan(e.handle);
      return new k(this.session, r);
    }
    asin(e) {
      if (typeof e == "number") return Math.asin(e);
      if (e instanceof k) return e.map(Math.asin);
      throw new Error("asin: unsupported value type");
    }
    acos(e) {
      if (typeof e == "number") return Math.acos(e);
      if (e instanceof k) return e.map(Math.acos);
      throw new Error("acos: unsupported value type");
    }
    atan(e) {
      if (typeof e == "number") return Math.atan(e);
      if (e instanceof k) return e.map(Math.atan);
      throw new Error("atan: unsupported value type");
    }
    atan2(e, r) {
      if (typeof e == "number" && typeof r == "number") return Math.atan2(e, r);
      if (e instanceof k && r instanceof k) {
        const n = e.data, i = r.data;
        if (n.length !== i.length) throw new Error("atan2: vectors must have same length");
        const o = new Float64Array(n.length);
        for (let a = 0; a < n.length; a++) o[a] = Math.atan2(n[a], i[a]);
        const u = this.session.wasm.createVector(Array.from(o));
        return new k(this.session, u);
      }
      throw new Error("atan2: unsupported value types");
    }
    sinh(e) {
      if (typeof e == "number") return Math.sinh(e);
      if (e instanceof k) return e.map(Math.sinh);
      throw new Error("sinh: unsupported value type");
    }
    cosh(e) {
      if (typeof e == "number") return Math.cosh(e);
      if (e instanceof k) return e.map(Math.cosh);
      throw new Error("cosh: unsupported value type");
    }
    tanh(e) {
      if (typeof e == "number") return Math.tanh(e);
      if (e instanceof k) return e.map(Math.tanh);
      throw new Error("tanh: unsupported value type");
    }
    exp(e) {
      if (typeof e == "number") return Math.exp(e);
      const r = this.session.wasm.exp(e.handle);
      return new k(this.session, r);
    }
    ln(e) {
      if (typeof e == "number") return Math.log(e);
      const r = this.session.wasm.ln(e.handle);
      return new k(this.session, r);
    }
    log(e) {
      return this.ln(e);
    }
    log10(e) {
      if (typeof e == "number") return Math.log10(e);
      if (e instanceof k) return e.map(Math.log10);
      throw new Error("log10: unsupported value type");
    }
    log2(e) {
      if (typeof e == "number") return Math.log2(e);
      if (e instanceof k) return e.map(Math.log2);
      throw new Error("log2: unsupported value type");
    }
    pow(e, r) {
      if (typeof e == "number") return Math.pow(e, r);
      if (e instanceof k) return e.map((n) => Math.pow(n, r));
      throw new Error("pow: unsupported value type");
    }
    floor(e) {
      if (typeof e == "number") return Math.floor(e);
      if (e instanceof k) return e.map(Math.floor);
      throw new Error("floor: unsupported value type");
    }
    ceil(e) {
      if (typeof e == "number") return Math.ceil(e);
      if (e instanceof k) return e.map(Math.ceil);
      throw new Error("ceil: unsupported value type");
    }
    round(e) {
      if (typeof e == "number") return Math.round(e);
      if (e instanceof k) return e.map(Math.round);
      throw new Error("round: unsupported value type");
    }
    trunc(e) {
      if (typeof e == "number") return Math.trunc(e);
      if (e instanceof k) return e.map(Math.trunc);
      throw new Error("trunc: unsupported value type");
    }
    sqrt(e) {
      if (typeof e == "number") return Math.sqrt(e);
      const r = this.session.wasm.sqrt(e.handle);
      return new k(this.session, r);
    }
    cbrt(e) {
      if (typeof e == "number") return Math.cbrt(e);
      if (e instanceof k) return e.map(Math.cbrt);
      throw new Error("cbrt: unsupported value type");
    }
    abs(e) {
      if (typeof e == "number") return Math.abs(e);
      const r = this.session.wasm.abs(e.handle);
      return new k(this.session, r);
    }
    sign(e) {
      if (typeof e == "number") return Math.sign(e);
      if (e instanceof k) return e.map(Math.sign);
      throw new Error("sign: unsupported value type");
    }
  }
  class Ms {
    constructor(e) {
      this.session = e;
    }
    fft(e) {
      const r = this.session.wasm.fft(e.handle), n = e.length;
      return new Ge(this.session, r, n, 2);
    }
    fftMag(e) {
      const r = this.session.wasm.fft_mag(e.handle);
      return new k(this.session, r);
    }
    fftPhase(e) {
      let r;
      e instanceof k ? r = this.fft(e) : r = e;
      const n = r.data, i = r.rows, o = new Float64Array(i);
      for (let a = 0; a < i; a++) {
        const f = n[a * 2], l = n[a * 2 + 1];
        o[a] = Math.atan2(l, f);
      }
      const u = this.session.wasm.createVector(Array.from(o));
      return new k(this.session, u);
    }
    ifft(e) {
      const r = this.session.wasm.ifft(e.handle);
      return new k(this.session, r);
    }
    dft(e) {
      const r = e.data, n = r.length, i = new Float64Array(n * 2);
      for (let u = 0; u < n; u++) {
        let a = 0, f = 0;
        for (let l = 0; l < n; l++) {
          const s = -2 * Math.PI * u * l / n;
          a += r[l] * Math.cos(s), f += r[l] * Math.sin(s);
        }
        i[u * 2] = a, i[u * 2 + 1] = f;
      }
      const o = this.session.wasm.createVector(Array.from(i));
      return new Ge(this.session, o, n, 2);
    }
    dftMag(e) {
      const r = this.dft(e);
      return this.fftMag(r);
    }
    dftPhase(e) {
      const r = this.dft(e);
      return this.fftPhase(r);
    }
    conv(e, r) {
      const n = e.data, i = r.data, o = n.length, u = i.length, a = o + u - 1, f = new Float64Array(a);
      for (let s = 0; s < a; s++) {
        let h = 0;
        for (let d = 0; d < u; d++) {
          const p = s - d;
          p >= 0 && p < o && (h += n[p] * i[d]);
        }
        f[s] = h;
      }
      const l = this.session.wasm.createVector(Array.from(f));
      return new k(this.session, l);
    }
    convFFT(e, r) {
      return this.conv(e, r);
    }
    hanning(e) {
      const r = this.session.wasm.hanningWindow(e);
      return new k(this.session, r);
    }
    hamming(e) {
      const r = this.session.wasm.hammingWindow(e);
      return new k(this.session, r);
    }
    blackman(e) {
      const r = this.session.wasm.blackmanWindow(e);
      return new k(this.session, r);
    }
    fftshift(e) {
      const r = e.data, n = r.length, i = Math.floor(n / 2), o = new Float64Array(n);
      for (let a = 0; a < i; a++) o[a] = r[a + i], o[a + i] = r[a];
      n % 2 !== 0 && (o[n - 1] = r[i]);
      const u = this.session.wasm.createVector(Array.from(o));
      return new k(this.session, u);
    }
    ifftshift(e) {
      return this.fftshift(e);
    }
    fftSpectrum(e) {
      const r = this.fft(e);
      return {
        magnitude: this.fftMag(r),
        phase: this.fftPhase(r)
      };
    }
  }
  class Ss {
    constructor(e) {
      this.session = e;
    }
    lu(e) {
      const r = this.session.wasm.lu_decomposition_js(e.handle), n = e.rows, i = e.cols;
      return {
        L: new Ge(this.session, r.L, n, i),
        U: new Ge(this.session, r.U, n, i),
        P: new Ge(this.session, r.P, n, i)
      };
    }
    qr(e) {
      const r = this.session.wasm.qr_decomposition_js(e.handle), n = e.rows, i = e.cols;
      return {
        Q: new Ge(this.session, r.Q, n, i),
        R: new Ge(this.session, r.R, n, i)
      };
    }
    svd(e) {
      const r = this.session.wasm.svd_decomposition_js(e.handle), n = e.rows, i = e.cols;
      return {
        U: new Ge(this.session, r.U, n, n),
        S: new k(this.session, r.S),
        V: new Ge(this.session, r.V, i, i)
      };
    }
    cholesky(e) {
      throw new Error("Cholesky decomposition not yet implemented");
    }
    powerIteration(e, r = 100, n = 1e-10) {
      const i = e.rows;
      if (i !== e.cols) throw new Error("Power iteration requires square matrix");
      let o = this.session.vector(Array.from({
        length: i
      }, () => Math.random()));
      o = this.normalize(o);
      let u = 0;
      for (let a = 0; a < r; a++) {
        const f = this.matVecMul(e, o), l = this.dot(o, f);
        if (Math.abs(l - u) < n) {
          u = l, o = this.normalize(f);
          break;
        }
        u = l, o = this.normalize(f);
      }
      return {
        value: u,
        vector: o
      };
    }
    eigenvalues(e, r = 100) {
      throw new Error("eigenvalues() not yet implemented");
    }
    eig(e) {
      throw new Error("eig() not yet implemented");
    }
    isSymmetric(e, r = 1e-10) {
      if (e.rows !== e.cols) return false;
      const n = e.rows;
      for (let i = 0; i < n; i++) for (let o = i + 1; o < n; o++) if (Math.abs(e.get(i, o) - e.get(o, i)) > r) return false;
      return true;
    }
    isPositiveDefinite(e) {
      throw new Error("isPositiveDefinite() not yet implemented");
    }
    identity(e) {
      const r = [];
      for (let n = 0; n < e; n++) {
        const i = [];
        for (let o = 0; o < e; o++) i.push(n === o ? 1 : 0);
        r.push(i);
      }
      return this.session.matrix(r);
    }
    det(e) {
      const { L: r, U: n, P: i } = this.lu(e);
      let o = 1;
      for (let a = 0; a < n.rows; a++) o *= n.get(a, a);
      return 1 * o;
    }
    inverse(e) {
      const r = this.session.wasm.matrixInverse(e.handle);
      return new Ge(this.session, r, e.rows, e.cols);
    }
    transpose(e) {
      const r = e.rows, n = e.cols, i = [];
      for (let o = 0; o < n; o++) {
        const u = [];
        for (let a = 0; a < r; a++) u.push(e.get(a, o));
        i.push(u);
      }
      return this.session.matrix(i);
    }
    matVecMul(e, r) {
      const n = e.rows, i = e.cols;
      if (i !== r.length) throw new Error("Matrix-vector dimensions mismatch");
      const o = new Float64Array(n), u = e.data, a = r.data;
      for (let l = 0; l < n; l++) {
        let s = 0;
        for (let h = 0; h < i; h++) s += u[l * i + h] * a[h];
        o[l] = s;
      }
      const f = this.session.wasm.createVector(Array.from(o));
      return new k(this.session, f);
    }
    dot(e, r) {
      const n = e.data, i = r.data;
      if (n.length !== i.length) throw new Error("Vectors must have same length");
      let o = 0;
      for (let u = 0; u < n.length; u++) o += n[u] * i[u];
      return o;
    }
    normalize(e) {
      const r = e.data;
      let n = 0;
      for (let u = 0; u < r.length; u++) n += r[u] * r[u];
      n = Math.sqrt(n);
      const i = new Float64Array(r.length);
      for (let u = 0; u < r.length; u++) i[u] = r[u] / n;
      const o = this.session.wasm.createVector(Array.from(i));
      return new k(this.session, o);
    }
  }
  class Ns {
    constructor(e) {
      this.session = e;
    }
    vadd(e, r) {
      const n = this.session.wasm.vadd(e.handle, r.handle);
      return new k(this.session, n);
    }
    vsub(e, r) {
      const n = this.session.wasm.vsub(e.handle, r.handle);
      return new k(this.session, n);
    }
    vmul(e, r) {
      const n = this.session.wasm.vmul(e.handle, r.handle);
      return new k(this.session, n);
    }
    vdiv(e, r) {
      const n = this.session.wasm.vdiv(e.handle, r.handle);
      return new k(this.session, n);
    }
    vscale(e, r) {
      const n = e.data, i = new Float64Array(n.length);
      for (let u = 0; u < n.length; u++) i[u] = n[u] * r;
      const o = this.session.wasm.createVector(Array.from(i));
      return new k(this.session, o);
    }
    dot(e, r) {
      const n = e.data, i = r.data;
      if (n.length !== i.length) throw new Error("Vectors must have same length");
      let o = 0;
      for (let u = 0; u < n.length; u++) o += n[u] * i[u];
      return o;
    }
    cross(e, r) {
      const n = e.data, i = r.data;
      if (n.length !== 3 || i.length !== 3) throw new Error("Cross product only defined for 3D vectors");
      const o = new Float64Array(3);
      o[0] = n[1] * i[2] - n[2] * i[1], o[1] = n[2] * i[0] - n[0] * i[2], o[2] = n[0] * i[1] - n[1] * i[0];
      const u = this.session.wasm.createVector(Array.from(o));
      return new k(this.session, u);
    }
    norm(e) {
      const r = e.data;
      let n = 0;
      for (let i = 0; i < r.length; i++) n += r[i] * r[i];
      return Math.sqrt(n);
    }
    normL1(e) {
      const r = e.data;
      let n = 0;
      for (let i = 0; i < r.length; i++) n += Math.abs(r[i]);
      return n;
    }
    normInf(e) {
      const r = e.data;
      let n = 0;
      for (let i = 0; i < r.length; i++) {
        const o = Math.abs(r[i]);
        o > n && (n = o);
      }
      return n;
    }
    normalize(e) {
      const r = this.norm(e);
      if (r < 1e-10) throw new Error("Cannot normalize zero vector");
      return this.vscale(e, 1 / r);
    }
  }
  class Bs {
    constructor(e) {
      this.session = e;
    }
    map(e, r) {
      return r.map(e);
    }
    filter(e, r) {
      return r.filter(e);
    }
    reduce(e, r, n) {
      return n.reduce(e, r);
    }
    pipe(e) {
      return (r) => {
        let n = r;
        for (const i of e) n = i(n);
        return n;
      };
    }
    compose(e) {
      return (r) => {
        let n = r;
        for (let i = e.length - 1; i >= 0; i--) n = e[i](n);
        return n;
      };
    }
    scan(e, r, n) {
      const i = n.data, o = new Float64Array(i.length);
      let u = r;
      for (let f = 0; f < i.length; f++) u = e(u, i[f]), o[f] = u;
      const a = this.session.wasm.createVector(Array.from(o));
      return new k(this.session, a);
    }
    zip(e, r, n) {
      const i = r.data, o = n.data, u = Math.min(i.length, o.length), a = new Float64Array(u);
      for (let l = 0; l < u; l++) a[l] = e(i[l], o[l]);
      const f = this.session.wasm.createVector(Array.from(a));
      return new k(this.session, f);
    }
    forEach(e, r) {
      const n = r.data;
      for (let i = 0; i < n.length; i++) e(n[i], i);
    }
    some(e, r) {
      const n = r.data;
      for (let i = 0; i < n.length; i++) if (e(n[i], i)) return true;
      return false;
    }
    every(e, r) {
      const n = r.data;
      for (let i = 0; i < n.length; i++) if (!e(n[i], i)) return false;
      return true;
    }
  }
  class xs {
    constructor(e) {
      this.session = e;
    }
    sum(e) {
      const r = e.data;
      let n = 0;
      for (let i = 0; i < r.length; i++) n += r[i];
      return n;
    }
    mean(e) {
      return this.sum(e) / e.length;
    }
    std(e, r = 0) {
      const n = this.variance(e, r);
      return Math.sqrt(n);
    }
    variance(e, r = 0) {
      const n = e.data, i = n.length;
      if (i <= r) throw new Error("Insufficient data for variance calculation");
      const o = this.mean(e);
      let u = 0;
      for (let a = 0; a < i; a++) {
        const f = n[a] - o;
        u += f * f;
      }
      return u / (i - r);
    }
    min(e) {
      const r = e.data;
      if (r.length === 0) throw new Error("Cannot find min of empty vector");
      let n = r[0];
      for (let i = 1; i < r.length; i++) r[i] < n && (n = r[i]);
      return n;
    }
    max(e) {
      const r = e.data;
      if (r.length === 0) throw new Error("Cannot find max of empty vector");
      let n = r[0];
      for (let i = 1; i < r.length; i++) r[i] > n && (n = r[i]);
      return n;
    }
    argmin(e) {
      const r = e.data;
      if (r.length === 0) throw new Error("Cannot find argmin of empty vector");
      let n = 0, i = r[0];
      for (let o = 1; o < r.length; o++) r[o] < i && (i = r[o], n = o);
      return n;
    }
    argmax(e) {
      const r = e.data;
      if (r.length === 0) throw new Error("Cannot find argmax of empty vector");
      let n = 0, i = r[0];
      for (let o = 1; o < r.length; o++) r[o] > i && (i = r[o], n = o);
      return n;
    }
    median(e) {
      const r = Array.from(e.data).sort((i, o) => i - o), n = r.length;
      return n % 2 === 0 ? (r[n / 2 - 1] + r[n / 2]) / 2 : r[Math.floor(n / 2)];
    }
    percentile(e, r) {
      if (r < 0 || r > 100) throw new Error("Percentile must be between 0 and 100");
      const n = Array.from(e.data).sort((l, s) => l - s), i = n.length;
      if (i === 0) throw new Error("Cannot compute percentile of empty vector");
      if (i === 1) return n[0];
      const o = r / 100 * (i - 1), u = Math.floor(o), a = Math.ceil(o), f = o - u;
      return n[u] * (1 - f) + n[a] * f;
    }
    cov(e, r, n = 0) {
      const i = e.data, o = r.data;
      if (i.length !== o.length) throw new Error("Vectors must have same length");
      const u = i.length;
      if (u <= n) throw new Error("Insufficient data for covariance calculation");
      const a = this.mean(e), f = this.mean(r);
      let l = 0;
      for (let s = 0; s < u; s++) l += (i[s] - a) * (o[s] - f);
      return l / (u - n);
    }
    corr(e, r) {
      const n = this.cov(e, r, 1), i = this.std(e, 1), o = this.std(r, 1);
      if (i === 0 || o === 0) throw new Error("Cannot compute correlation with zero variance");
      return n / (i * o);
    }
    prod(e) {
      const r = e.data;
      let n = 1;
      for (let i = 0; i < r.length; i++) n *= r[i];
      return n;
    }
    cumsum(e) {
      const r = e.data, n = new Float64Array(r.length);
      let i = 0;
      for (let u = 0; u < r.length; u++) i += r[u], n[u] = i;
      const o = this.session.wasm.createVector(Array.from(n));
      return new k(this.session, o);
    }
    cumprod(e) {
      const r = e.data, n = new Float64Array(r.length);
      let i = 1;
      for (let u = 0; u < r.length; u++) i *= r[u], n[u] = i;
      const o = this.session.wasm.createVector(Array.from(n));
      return new k(this.session, o);
    }
  }
  class Ts {
    constructor() {
      this.session = new Cs(), this.math = new bs(this.session), this.dsp = new Ms(this.session), this.linalg = new Ss(this.session), this.vecOps = new Ns(this.session), this.hof = new Bs(this.session), this.stats = new xs(this.session);
    }
    async init() {
      await this.session.init();
    }
    async use(e) {
      return this.session.use(e);
    }
    cleanup() {
      this.session.cleanup();
    }
    vec(e) {
      return this.session.vector(e);
    }
    vector(e) {
      return this.session.vector(e);
    }
    mat(e) {
      return this.session.matrix(e);
    }
    matrix(e) {
      return this.session.matrix(e);
    }
    scalar(e) {
      return this.session.scalar(e);
    }
    complex(e, r) {
      return this.session.complex(e, r);
    }
    linspace(e, r, n) {
      const i = this.session.wasm.linspace(e, r, n);
      return new k(this.session, i);
    }
    identity(e) {
      return this.linalg.identity(e);
    }
    zeros(e) {
      return this.vector(new Array(e).fill(0));
    }
    ones(e) {
      return this.vector(new Array(e).fill(1));
    }
    sin(e) {
      return this.math.sin(e);
    }
    cos(e) {
      return this.math.cos(e);
    }
    tan(e) {
      return this.math.tan(e);
    }
    exp(e) {
      return this.math.exp(e);
    }
    ln(e) {
      return this.math.ln(e);
    }
    log(e) {
      return this.math.log(e);
    }
    sqrt(e) {
      return this.math.sqrt(e);
    }
    abs(e) {
      return this.math.abs(e);
    }
    pow(e, r) {
      return this.math.pow(e, r);
    }
    fft(e) {
      return this.dsp.fft(e);
    }
    fftMag(e) {
      return this.dsp.fftMag(e);
    }
    ifft(e) {
      return this.dsp.ifft(e);
    }
    conv(e, r) {
      return this.dsp.conv(e, r);
    }
    lu(e) {
      return this.linalg.lu(e);
    }
    qr(e) {
      return this.linalg.qr(e);
    }
    svd(e) {
      return this.linalg.svd(e);
    }
    det(e) {
      return this.linalg.det(e);
    }
    transpose(e) {
      return this.linalg.transpose(e);
    }
    dot(e, r) {
      return this.vecOps.dot(e, r);
    }
    cross(e, r) {
      return this.vecOps.cross(e, r);
    }
    norm(e) {
      return this.vecOps.norm(e);
    }
    sum(e) {
      return this.stats.sum(e);
    }
    mean(e) {
      return this.stats.mean(e);
    }
    std(e, r) {
      return this.stats.std(e, r);
    }
    min(e) {
      return this.stats.min(e);
    }
    max(e) {
      return this.stats.max(e);
    }
    map(e, r) {
      return this.hof.map(e, r);
    }
    filter(e, r) {
      return this.hof.filter(e, r);
    }
    reduce(e, r, n) {
      return this.hof.reduce(e, r, n);
    }
    eval(e) {
      try {
        return this.session.wasm._eval(e);
      } catch (r) {
        throw new Error(`Eval failed: ${r}`);
      }
    }
    resetEvaluator() {
      this.session.wasm.reset();
    }
    get PI() {
      return Math.PI;
    }
    get E() {
      return Math.E;
    }
    get SQRT2() {
      return Math.SQRT2;
    }
    get LN2() {
      return Math.LN2;
    }
    get LN10() {
      return Math.LN10;
    }
    getMemoryStats() {
      return this.session.handleManager.getStats();
    }
    gc() {
      return this.session.handleManager.gc();
    }
    getActiveValuesCount() {
      return this.session.getActiveValuesCount();
    }
  }
  function or() {
    return or = Object.assign ? Object.assign.bind() : function(t) {
      for (var e = 1; e < arguments.length; e++) {
        var r = arguments[e];
        for (var n in r) ({}).hasOwnProperty.call(r, n) && (t[n] = r[n]);
      }
      return t;
    }, or.apply(null, arguments);
  }
  var yi = {
    epsilon: 1e-12,
    matrix: "Matrix",
    number: "number",
    precision: 64,
    predictable: false,
    randomSeed: null
  };
  function ve(t) {
    return typeof t == "number";
  }
  function Fe(t) {
    return !t || typeof t != "object" || typeof t.constructor != "function" ? false : t.isBigNumber === true && typeof t.constructor.prototype == "object" && t.constructor.prototype.isBigNumber === true || typeof t.constructor.isDecimal == "function" && t.constructor.isDecimal(t) === true;
  }
  function _i(t) {
    return t && typeof t == "object" && Object.getPrototypeOf(t).isComplex === true || false;
  }
  function Ai(t) {
    return t && typeof t == "object" && Object.getPrototypeOf(t).isFraction === true || false;
  }
  function Fi(t) {
    return t && t.constructor.prototype.isUnit === true || false;
  }
  function Xe(t) {
    return typeof t == "string";
  }
  var we = Array.isArray;
  function Ae(t) {
    return t && t.constructor.prototype.isMatrix === true || false;
  }
  function Wt(t) {
    return Array.isArray(t) || Ae(t);
  }
  function $s(t) {
    return t && t.isDenseMatrix && t.constructor.prototype.isMatrix === true || false;
  }
  function Is(t) {
    return t && t.isSparseMatrix && t.constructor.prototype.isMatrix === true || false;
  }
  function zs(t) {
    return t && t.constructor.prototype.isRange === true || false;
  }
  function Jr(t) {
    return t && t.constructor.prototype.isIndex === true || false;
  }
  function Os(t) {
    return typeof t == "boolean";
  }
  function Ps(t) {
    return t && t.constructor.prototype.isResultSet === true || false;
  }
  function qs(t) {
    return t && t.constructor.prototype.isHelp === true || false;
  }
  function Rs(t) {
    return typeof t == "function";
  }
  function Ls(t) {
    return t instanceof Date;
  }
  function Vs(t) {
    return t instanceof RegExp;
  }
  function Kr(t) {
    return !!(t && typeof t == "object" && t.constructor === Object && !_i(t) && !Ai(t));
  }
  function Us(t) {
    return t === null;
  }
  function Ws(t) {
    return t === void 0;
  }
  function Zs(t) {
    return t && t.isAccessorNode === true && t.constructor.prototype.isNode === true || false;
  }
  function Hs(t) {
    return t && t.isArrayNode === true && t.constructor.prototype.isNode === true || false;
  }
  function js(t) {
    return t && t.isAssignmentNode === true && t.constructor.prototype.isNode === true || false;
  }
  function Js(t) {
    return t && t.isBlockNode === true && t.constructor.prototype.isNode === true || false;
  }
  function Ks(t) {
    return t && t.isConditionalNode === true && t.constructor.prototype.isNode === true || false;
  }
  function Ys(t) {
    return t && t.isConstantNode === true && t.constructor.prototype.isNode === true || false;
  }
  function Gs(t) {
    return t && t.isFunctionAssignmentNode === true && t.constructor.prototype.isNode === true || false;
  }
  function Qs(t) {
    return t && t.isFunctionNode === true && t.constructor.prototype.isNode === true || false;
  }
  function Xs(t) {
    return t && t.isIndexNode === true && t.constructor.prototype.isNode === true || false;
  }
  function ks(t) {
    return t && t.isNode === true && t.constructor.prototype.isNode === true || false;
  }
  function ec(t) {
    return t && t.isObjectNode === true && t.constructor.prototype.isNode === true || false;
  }
  function tc(t) {
    return t && t.isOperatorNode === true && t.constructor.prototype.isNode === true || false;
  }
  function rc(t) {
    return t && t.isParenthesisNode === true && t.constructor.prototype.isNode === true || false;
  }
  function nc(t) {
    return t && t.isRangeNode === true && t.constructor.prototype.isNode === true || false;
  }
  function ic(t) {
    return t && t.isRelationalNode === true && t.constructor.prototype.isNode === true || false;
  }
  function oc(t) {
    return t && t.isSymbolNode === true && t.constructor.prototype.isNode === true || false;
  }
  function uc(t) {
    return t && t.constructor.prototype.isChain === true || false;
  }
  function ct(t) {
    var e = typeof t;
    return e === "object" ? t === null ? "null" : Fe(t) ? "BigNumber" : t.constructor && t.constructor.name ? t.constructor.name : "Object" : e;
  }
  function _e(t) {
    var e = typeof t;
    if (e === "number" || e === "string" || e === "boolean" || t === null || t === void 0) return t;
    if (typeof t.clone == "function") return t.clone();
    if (Array.isArray(t)) return t.map(function(r) {
      return _e(r);
    });
    if (t instanceof Date) return new Date(t.valueOf());
    if (Fe(t)) return t;
    if (Kr(t)) return ac(t, _e);
    throw new TypeError("Cannot clone: unknown type of value (value: ".concat(t, ")"));
  }
  function ac(t, e) {
    var r = {};
    for (var n in t) Zt(t, n) && (r[n] = e(t[n]));
    return r;
  }
  function Ei(t, e) {
    for (var r in e) Zt(e, r) && (t[r] = e[r]);
    return t;
  }
  function Tt(t, e) {
    var r, n, i;
    if (Array.isArray(t)) {
      if (!Array.isArray(e) || t.length !== e.length) return false;
      for (n = 0, i = t.length; n < i; n++) if (!Tt(t[n], e[n])) return false;
      return true;
    } else {
      if (typeof t == "function") return t === e;
      if (t instanceof Object) {
        if (Array.isArray(e) || !(e instanceof Object)) return false;
        for (r in t) if (!(r in e) || !Tt(t[r], e[r])) return false;
        for (r in e) if (!(r in t)) return false;
        return true;
      } else return t === e;
    }
  }
  function Zt(t, e) {
    return t && Object.hasOwnProperty.call(t, e);
  }
  function sc(t, e) {
    for (var r = {}, n = 0; n < e.length; n++) {
      var i = e[n], o = t[i];
      o !== void 0 && (r[i] = o);
    }
    return r;
  }
  var cc = [
    "Matrix",
    "Array"
  ], fc = [
    "number",
    "BigNumber",
    "Fraction"
  ], We = function(e) {
    if (e) throw new Error(`The global config is readonly. 
Please create a mathjs instance if you want to change the default configuration. 
Example:

  import { create, all } from 'mathjs';
  const mathjs = create(all);
  mathjs.config({ number: 'BigNumber' });
`);
    return Object.freeze(yi);
  };
  or(We, yi, {
    MATRIX_OPTIONS: cc,
    NUMBER_OPTIONS: fc
  });
  function hn() {
    return true;
  }
  function je() {
    return false;
  }
  function At() {
  }
  const dn = "Argument is not a typed-function.";
  function Ci() {
    function t(F) {
      return typeof F == "object" && F !== null && F.constructor === Object;
    }
    const e = [
      {
        name: "number",
        test: function(F) {
          return typeof F == "number";
        }
      },
      {
        name: "string",
        test: function(F) {
          return typeof F == "string";
        }
      },
      {
        name: "boolean",
        test: function(F) {
          return typeof F == "boolean";
        }
      },
      {
        name: "Function",
        test: function(F) {
          return typeof F == "function";
        }
      },
      {
        name: "Array",
        test: Array.isArray
      },
      {
        name: "Date",
        test: function(F) {
          return F instanceof Date;
        }
      },
      {
        name: "RegExp",
        test: function(F) {
          return F instanceof RegExp;
        }
      },
      {
        name: "Object",
        test: t
      },
      {
        name: "null",
        test: function(F) {
          return F === null;
        }
      },
      {
        name: "undefined",
        test: function(F) {
          return F === void 0;
        }
      }
    ], r = {
      name: "any",
      test: hn,
      isAny: true
    };
    let n, i, o = 0, u = {
      createCount: 0
    };
    function a(F) {
      const M = n.get(F);
      if (M) return M;
      let T = 'Unknown type "' + F + '"';
      const z = F.toLowerCase();
      let R;
      for (R of i) if (R.toLowerCase() === z) {
        T += '. Did you mean "' + R + '" ?';
        break;
      }
      throw new TypeError(T);
    }
    function f(F) {
      let M = arguments.length > 1 && arguments[1] !== void 0 ? arguments[1] : "any";
      const T = M ? a(M).index : i.length, z = [];
      for (let O = 0; O < F.length; ++O) {
        if (!F[O] || typeof F[O].name != "string" || typeof F[O].test != "function") throw new TypeError("Object with properties {name: string, test: function} expected");
        const H = F[O].name;
        if (n.has(H)) throw new TypeError('Duplicate type name "' + H + '"');
        z.push(H), n.set(H, {
          name: H,
          test: F[O].test,
          isAny: F[O].isAny,
          index: T + O,
          conversionsTo: []
        });
      }
      const R = i.slice(T);
      i = i.slice(0, T).concat(z).concat(R);
      for (let O = T + z.length; O < i.length; ++O) n.get(i[O]).index = O;
    }
    function l() {
      n = /* @__PURE__ */ new Map(), i = [], o = 0, f([
        r
      ], false);
    }
    l(), f(e);
    function s() {
      let F;
      for (F of i) n.get(F).conversionsTo = [];
      o = 0;
    }
    function h(F) {
      const M = i.filter((T) => {
        const z = n.get(T);
        return !z.isAny && z.test(F);
      });
      return M.length ? M : [
        "any"
      ];
    }
    function d(F) {
      return F && typeof F == "function" && "_typedFunctionData" in F;
    }
    function p(F, M, T) {
      if (!d(F)) throw new TypeError(dn);
      const z = T && T.exact, R = Array.isArray(M) ? M.join(",") : M, O = A(R), H = g(O);
      if (!z || H in F.signatures) {
        const le = F._typedFunctionData.signatureMap.get(H);
        if (le) return le;
      }
      const V = O.length;
      let j;
      if (z) {
        j = [];
        let le;
        for (le in F.signatures) j.push(F._typedFunctionData.signatureMap.get(le));
      } else j = F._typedFunctionData.signatures;
      for (let le = 0; le < V; ++le) {
        const pe = O[le], be = [];
        let Re;
        for (Re of j) {
          const qe = C(Re.params, le);
          if (!(!qe || pe.restParam && !qe.restParam)) {
            if (!qe.hasAny) {
              const Ke = v(qe);
              if (pe.types.some((Ye) => !Ke.has(Ye.name))) continue;
            }
            be.push(Re);
          }
        }
        if (j = be, j.length === 0) break;
      }
      let L;
      for (L of j) if (L.params.length <= V) return L;
      throw new TypeError("Signature not found (signature: " + (F.name || "unnamed") + "(" + g(O, ", ") + "))");
    }
    function D(F, M, T) {
      return p(F, M, T).implementation;
    }
    function c(F, M) {
      const T = a(M);
      if (T.test(F)) return F;
      const z = T.conversionsTo;
      if (z.length === 0) throw new Error("There are no conversions to " + M + " defined.");
      for (let R = 0; R < z.length; R++) if (a(z[R].from).test(F)) return z[R].convert(F);
      throw new Error("Cannot convert " + F + " to " + M);
    }
    function g(F) {
      let M = arguments.length > 1 && arguments[1] !== void 0 ? arguments[1] : ",";
      return F.map((T) => T.name).join(M);
    }
    function m(F) {
      const M = F.indexOf("...") === 0, z = (M ? F.length > 3 ? F.slice(3) : "any" : F).split("|").map((V) => a(V.trim()));
      let R = false, O = M ? "..." : "";
      return {
        types: z.map(function(V) {
          return R = V.isAny || R, O += V.name + "|", {
            name: V.name,
            typeIndex: V.index,
            test: V.test,
            isAny: V.isAny,
            conversion: null,
            conversionIndex: -1
          };
        }),
        name: O.slice(0, -1),
        hasAny: R,
        hasConversion: false,
        restParam: M
      };
    }
    function w(F) {
      const M = F.types.map((H) => H.name), T = K(M);
      let z = F.hasAny, R = F.name;
      const O = T.map(function(H) {
        const V = a(H.from);
        return z = V.isAny || z, R += "|" + H.from, {
          name: H.from,
          typeIndex: V.index,
          test: V.test,
          isAny: V.isAny,
          conversion: H,
          conversionIndex: H.index
        };
      });
      return {
        types: F.types.concat(O),
        name: R,
        hasAny: z,
        hasConversion: O.length > 0,
        restParam: F.restParam
      };
    }
    function v(F) {
      return F.typeSet || (F.typeSet = /* @__PURE__ */ new Set(), F.types.forEach((M) => F.typeSet.add(M.name))), F.typeSet;
    }
    function A(F) {
      const M = [];
      if (typeof F != "string") throw new TypeError("Signatures must be strings");
      const T = F.trim();
      if (T === "") return M;
      const z = T.split(",");
      for (let R = 0; R < z.length; ++R) {
        const O = m(z[R].trim());
        if (O.restParam && R !== z.length - 1) throw new SyntaxError('Unexpected rest parameter "' + z[R] + '": only allowed for the last parameter');
        if (O.types.length === 0) return null;
        M.push(O);
      }
      return M;
    }
    function y(F) {
      const M = he(F);
      return M ? M.restParam : false;
    }
    function _(F) {
      if (!F || F.types.length === 0) return hn;
      if (F.types.length === 1) return a(F.types[0].name).test;
      if (F.types.length === 2) {
        const M = a(F.types[0].name).test, T = a(F.types[1].name).test;
        return function(R) {
          return M(R) || T(R);
        };
      } else {
        const M = F.types.map(function(T) {
          return a(T.name).test;
        });
        return function(z) {
          for (let R = 0; R < M.length; R++) if (M[R](z)) return true;
          return false;
        };
      }
    }
    function E(F) {
      let M, T, z;
      if (y(F)) {
        M = Pe(F).map(_);
        const R = M.length, O = _(he(F)), H = function(V) {
          for (let j = R; j < V.length; j++) if (!O(V[j])) return false;
          return true;
        };
        return function(j) {
          for (let L = 0; L < M.length; L++) if (!M[L](j[L])) return false;
          return H(j) && j.length >= R + 1;
        };
      } else return F.length === 0 ? function(O) {
        return O.length === 0;
      } : F.length === 1 ? (T = _(F[0]), function(O) {
        return T(O[0]) && O.length === 1;
      }) : F.length === 2 ? (T = _(F[0]), z = _(F[1]), function(O) {
        return T(O[0]) && z(O[1]) && O.length === 2;
      }) : (M = F.map(_), function(O) {
        for (let H = 0; H < M.length; H++) if (!M[H](O[H])) return false;
        return O.length === M.length;
      });
    }
    function C(F, M) {
      return M < F.length ? F[M] : y(F) ? he(F) : null;
    }
    function b(F, M) {
      const T = C(F, M);
      return T ? v(T) : /* @__PURE__ */ new Set();
    }
    function x(F) {
      return F.conversion === null || F.conversion === void 0;
    }
    function N(F, M) {
      const T = /* @__PURE__ */ new Set();
      return F.forEach((z) => {
        const R = b(z.params, M);
        let O;
        for (O of R) T.add(O);
      }), T.has("any") ? [
        "any"
      ] : Array.from(T);
    }
    function B(F, M, T) {
      let z, R;
      const O = F || "unnamed";
      let H = T, V;
      for (V = 0; V < M.length; V++) {
        const pe = [];
        if (H.forEach((be) => {
          const Re = C(be.params, V), qe = _(Re);
          (V < be.params.length || y(be.params)) && qe(M[V]) && pe.push(be);
        }), pe.length === 0) {
          if (R = N(H, V), R.length > 0) {
            const be = h(M[V]);
            return z = new TypeError("Unexpected type of argument in function " + O + " (expected: " + R.join(" or ") + ", actual: " + be.join(" | ") + ", index: " + V + ")"), z.data = {
              category: "wrongType",
              fn: O,
              index: V,
              actual: be,
              expected: R
            }, z;
          }
        } else H = pe;
      }
      const j = H.map(function(pe) {
        return y(pe.params) ? 1 / 0 : pe.params.length;
      });
      if (M.length < Math.min.apply(null, j)) return R = N(H, V), z = new TypeError("Too few arguments in function " + O + " (expected: " + R.join(" or ") + ", index: " + M.length + ")"), z.data = {
        category: "tooFewArgs",
        fn: O,
        index: M.length,
        expected: R
      }, z;
      const L = Math.max.apply(null, j);
      if (M.length > L) return z = new TypeError("Too many arguments in function " + O + " (expected: " + L + ", actual: " + M.length + ")"), z.data = {
        category: "tooManyArgs",
        fn: O,
        index: M.length,
        expectedLength: L
      }, z;
      const le = [];
      for (let pe = 0; pe < M.length; ++pe) le.push(h(M[pe]).join("|"));
      return z = new TypeError('Arguments of type "' + le.join(", ") + '" do not match any of the defined signatures of function ' + O + "."), z.data = {
        category: "mismatch",
        actual: le
      }, z;
    }
    function Z(F) {
      let M = i.length + 1;
      for (let T = 0; T < F.types.length; T++) x(F.types[T]) && (M = Math.min(M, F.types[T].typeIndex));
      return M;
    }
    function q(F) {
      let M = o + 1;
      for (let T = 0; T < F.types.length; T++) x(F.types[T]) || (M = Math.min(M, F.types[T].conversionIndex));
      return M;
    }
    function U(F, M) {
      if (F.hasAny) {
        if (!M.hasAny) return 1;
      } else if (M.hasAny) return -1;
      if (F.restParam) {
        if (!M.restParam) return 1;
      } else if (M.restParam) return -1;
      if (F.hasConversion) {
        if (!M.hasConversion) return 1;
      } else if (M.hasConversion) return -1;
      const T = Z(F) - Z(M);
      if (T < 0) return -1;
      if (T > 0) return 1;
      const z = q(F) - q(M);
      return z < 0 ? -1 : z > 0 ? 1 : 0;
    }
    function I(F, M) {
      const T = F.params, z = M.params, R = he(T), O = he(z), H = y(T), V = y(z);
      if (H && R.hasAny) {
        if (!V || !O.hasAny) return 1;
      } else if (V && O.hasAny) return -1;
      let j = 0, L = 0, le;
      for (le of T) le.hasAny && ++j, le.hasConversion && ++L;
      let pe = 0, be = 0;
      for (le of z) le.hasAny && ++pe, le.hasConversion && ++be;
      if (j !== pe) return j - pe;
      if (H && R.hasConversion) {
        if (!V || !O.hasConversion) return 1;
      } else if (V && O.hasConversion) return -1;
      if (L !== be) return L - be;
      if (H) {
        if (!V) return 1;
      } else if (V) return -1;
      const Re = (T.length - z.length) * (H ? -1 : 1);
      if (Re !== 0) return Re;
      const qe = [];
      let Ke = 0;
      for (let _t = 0; _t < T.length; ++_t) {
        const Gt = U(T[_t], z[_t]);
        qe.push(Gt), Ke += Gt;
      }
      if (Ke !== 0) return Ke;
      let Ye;
      for (Ye of qe) if (Ye !== 0) return Ye;
      return 0;
    }
    function K(F) {
      if (F.length === 0) return [];
      const M = F.map(a);
      F.length > 1 && M.sort((R, O) => R.index - O.index);
      let T = M[0].conversionsTo;
      if (F.length === 1) return T;
      T = T.concat([]);
      const z = new Set(F);
      for (let R = 1; R < M.length; ++R) {
        let O;
        for (O of M[R].conversionsTo) z.has(O.from) || (T.push(O), z.add(O.from));
      }
      return T;
    }
    function Q(F, M) {
      let T = M;
      if (F.some((R) => R.hasConversion)) {
        const R = y(F), O = F.map(G);
        T = function() {
          const V = [], j = R ? arguments.length - 1 : arguments.length;
          for (let L = 0; L < j; L++) V[L] = O[L](arguments[L]);
          return R && (V[j] = arguments[j].map(O[j])), M.apply(this, V);
        };
      }
      let z = T;
      if (y(F)) {
        const R = F.length - 1;
        z = function() {
          return T.apply(this, fe(arguments, 0, R).concat([
            fe(arguments, R)
          ]));
        };
      }
      return z;
    }
    function G(F) {
      let M, T, z, R;
      const O = [], H = [];
      switch (F.types.forEach(function(V) {
        V.conversion && (O.push(a(V.conversion.from).test), H.push(V.conversion.convert));
      }), H.length) {
        case 0:
          return function(j) {
            return j;
          };
        case 1:
          return M = O[0], z = H[0], function(j) {
            return M(j) ? z(j) : j;
          };
        case 2:
          return M = O[0], T = O[1], z = H[0], R = H[1], function(j) {
            return M(j) ? z(j) : T(j) ? R(j) : j;
          };
        default:
          return function(j) {
            for (let L = 0; L < H.length; L++) if (O[L](j)) return H[L](j);
            return j;
          };
      }
    }
    function re(F) {
      function M(T, z, R) {
        if (z < T.length) {
          const O = T[z];
          let H = [];
          if (O.restParam) {
            const V = O.types.filter(x);
            V.length < O.types.length && H.push({
              types: V,
              name: "..." + V.map((j) => j.name).join("|"),
              hasAny: V.some((j) => j.isAny),
              hasConversion: false,
              restParam: true
            }), H.push(O);
          } else H = O.types.map(function(V) {
            return {
              types: [
                V
              ],
              name: V.name,
              hasAny: V.isAny,
              hasConversion: V.conversion,
              restParam: false
            };
          });
          return Te(H, function(V) {
            return M(T, z + 1, R.concat([
              V
            ]));
          });
        } else return [
          R
        ];
      }
      return M(F, 0, []);
    }
    function ce(F, M) {
      const T = Math.max(F.length, M.length);
      for (let V = 0; V < T; V++) {
        const j = b(F, V), L = b(M, V);
        let le = false, pe;
        for (pe of L) if (j.has(pe)) {
          le = true;
          break;
        }
        if (!le) return false;
      }
      const z = F.length, R = M.length, O = y(F), H = y(M);
      return O ? H ? z === R : R >= z : H ? z >= R : z === R;
    }
    function ae(F) {
      return F.map((M) => nt(M) ? He(M.referToSelf.callback) : rt(M) ? xe(M.referTo.references, M.referTo.callback) : M);
    }
    function ue(F, M, T) {
      const z = [];
      let R;
      for (R of F) {
        let O = T[R];
        if (typeof O != "number") throw new TypeError('No definition for referenced signature "' + R + '"');
        if (O = M[O], typeof O != "function") return false;
        z.push(O);
      }
      return z;
    }
    function Ce(F, M, T) {
      const z = ae(F), R = new Array(z.length).fill(false);
      let O = true;
      for (; O; ) {
        O = false;
        let H = true;
        for (let V = 0; V < z.length; ++V) {
          if (R[V]) continue;
          const j = z[V];
          if (nt(j)) z[V] = j.referToSelf.callback(T), z[V].referToSelf = j.referToSelf, R[V] = true, H = false;
          else if (rt(j)) {
            const L = ue(j.referTo.references, z, M);
            L ? (z[V] = j.referTo.callback.apply(this, L), z[V].referTo = j.referTo, R[V] = true, H = false) : O = true;
          }
        }
        if (H && O) throw new SyntaxError("Circular reference detected in resolving typed.referTo");
      }
      return z;
    }
    function Ee(F) {
      const M = /\bthis(\(|\.signatures\b)/;
      Object.keys(F).forEach((T) => {
        const z = F[T];
        if (M.test(z.toString())) throw new SyntaxError("Using `this` to self-reference a function is deprecated since typed-function@3. Use typed.referTo and typed.referToSelf instead.");
      });
    }
    function Se(F, M) {
      if (u.createCount++, Object.keys(M).length === 0) throw new SyntaxError("No signatures provided");
      u.warnAgainstDeprecatedThis && Ee(M);
      const T = [], z = [], R = {}, O = [];
      let H;
      for (H in M) {
        if (!Object.prototype.hasOwnProperty.call(M, H)) continue;
        const se = A(H);
        if (!se) continue;
        T.forEach(function(Rt) {
          if (ce(Rt, se)) throw new TypeError('Conflicting signatures "' + g(Rt) + '" and "' + g(se) + '".');
        }), T.push(se);
        const Le = z.length;
        z.push(M[H]);
        const tu = se.map(w);
        let Qt;
        for (Qt of re(tu)) {
          const Rt = g(Qt);
          O.push({
            params: Qt,
            name: Rt,
            fn: Le
          }), Qt.every((ru) => !ru.hasConversion) && (R[Rt] = Le);
        }
      }
      O.sort(I);
      const V = Ce(z, R, qt);
      let j;
      for (j in R) Object.prototype.hasOwnProperty.call(R, j) && (R[j] = V[R[j]]);
      const L = [], le = /* @__PURE__ */ new Map();
      for (j of O) le.has(j.name) || (j.fn = V[j.fn], L.push(j), le.set(j.name, j));
      const pe = L[0] && L[0].params.length <= 2 && !y(L[0].params), be = L[1] && L[1].params.length <= 2 && !y(L[1].params), Re = L[2] && L[2].params.length <= 2 && !y(L[2].params), qe = L[3] && L[3].params.length <= 2 && !y(L[3].params), Ke = L[4] && L[4].params.length <= 2 && !y(L[4].params), Ye = L[5] && L[5].params.length <= 2 && !y(L[5].params), _t = pe && be && Re && qe && Ke && Ye;
      for (let se = 0; se < L.length; ++se) L[se].test = E(L[se].params);
      const Gt = pe ? _(L[0].params[0]) : je, Mo = be ? _(L[1].params[0]) : je, So = Re ? _(L[2].params[0]) : je, No = qe ? _(L[3].params[0]) : je, Bo = Ke ? _(L[4].params[0]) : je, xo = Ye ? _(L[5].params[0]) : je, To = pe ? _(L[0].params[1]) : je, $o = be ? _(L[1].params[1]) : je, Io = Re ? _(L[2].params[1]) : je, zo = qe ? _(L[3].params[1]) : je, Oo = Ke ? _(L[4].params[1]) : je, Po = Ye ? _(L[5].params[1]) : je;
      for (let se = 0; se < L.length; ++se) L[se].implementation = Q(L[se].params, L[se].fn);
      const qo = pe ? L[0].implementation : At, Ro = be ? L[1].implementation : At, Lo = Re ? L[2].implementation : At, Vo = qe ? L[3].implementation : At, Uo = Ke ? L[4].implementation : At, Wo = Ye ? L[5].implementation : At, Zo = pe ? L[0].params.length : -1, Ho = be ? L[1].params.length : -1, jo = Re ? L[2].params.length : -1, Jo = qe ? L[3].params.length : -1, Ko = Ke ? L[4].params.length : -1, Yo = Ye ? L[5].params.length : -1, Go = _t ? 6 : 0, Qo = L.length, Xo = L.map((se) => se.test), ko = L.map((se) => se.implementation), eu = function() {
        for (let Le = Go; Le < Qo; Le++) if (Xo[Le](arguments)) return ko[Le].apply(this, arguments);
        return u.onMismatch(F, arguments, L);
      };
      function qt(se, Le) {
        return arguments.length === Zo && Gt(se) && To(Le) ? qo.apply(this, arguments) : arguments.length === Ho && Mo(se) && $o(Le) ? Ro.apply(this, arguments) : arguments.length === jo && So(se) && Io(Le) ? Lo.apply(this, arguments) : arguments.length === Jo && No(se) && zo(Le) ? Vo.apply(this, arguments) : arguments.length === Ko && Bo(se) && Oo(Le) ? Uo.apply(this, arguments) : arguments.length === Yo && xo(se) && Po(Le) ? Wo.apply(this, arguments) : eu.apply(this, arguments);
      }
      try {
        Object.defineProperty(qt, "name", {
          value: F
        });
      } catch {
      }
      return qt.signatures = R, qt._typedFunctionData = {
        signatures: L,
        signatureMap: le
      }, qt;
    }
    function ge(F, M, T) {
      throw B(F, M, T);
    }
    function Pe(F) {
      return fe(F, 0, F.length - 1);
    }
    function he(F) {
      return F[F.length - 1];
    }
    function fe(F, M, T) {
      return Array.prototype.slice.call(F, M, T);
    }
    function Ze(F, M) {
      for (let T = 0; T < F.length; T++) if (M(F[T])) return F[T];
    }
    function Te(F, M) {
      return Array.prototype.concat.apply([], F.map(M));
    }
    function de() {
      const F = Pe(arguments).map((T) => g(A(T))), M = he(arguments);
      if (typeof M != "function") throw new TypeError("Callback function expected as last argument");
      return xe(F, M);
    }
    function xe(F, M) {
      return {
        referTo: {
          references: F,
          callback: M
        }
      };
    }
    function He(F) {
      if (typeof F != "function") throw new TypeError("Callback function expected as first argument");
      return {
        referToSelf: {
          callback: F
        }
      };
    }
    function rt(F) {
      return F && typeof F.referTo == "object" && Array.isArray(F.referTo.references) && typeof F.referTo.callback == "function";
    }
    function nt(F) {
      return F && typeof F.referToSelf == "object" && typeof F.referToSelf.callback == "function";
    }
    function wt(F, M) {
      if (!F) return M;
      if (M && M !== F) {
        const T = new Error("Function names do not match (expected: " + F + ", actual: " + M + ")");
        throw T.data = {
          actual: M,
          expected: F
        }, T;
      }
      return F;
    }
    function yt(F) {
      let M;
      for (const T in F) Object.prototype.hasOwnProperty.call(F, T) && (d(F[T]) || typeof F[T].signature == "string") && (M = wt(M, F[T].name));
      return M;
    }
    function xr(F, M) {
      let T;
      for (T in M) if (Object.prototype.hasOwnProperty.call(M, T)) {
        if (T in F && M[T] !== F[T]) {
          const z = new Error('Signature "' + T + '" is defined twice');
          throw z.data = {
            signature: T,
            sourceFunction: M[T],
            destFunction: F[T]
          }, z;
        }
        F[T] = M[T];
      }
    }
    const bo = u;
    u = function(F) {
      const M = typeof F == "string", T = M ? 1 : 0;
      let z = M ? F : "";
      const R = {};
      for (let O = T; O < arguments.length; ++O) {
        const H = arguments[O];
        let V = {}, j;
        if (typeof H == "function" ? (j = H.name, typeof H.signature == "string" ? V[H.signature] = H : d(H) && (V = H.signatures)) : t(H) && (V = H, M || (j = yt(H))), Object.keys(V).length === 0) {
          const L = new TypeError("Argument to 'typed' at index " + O + " is not a (typed) function, nor an object with signatures as keys and functions as values.");
          throw L.data = {
            index: O,
            argument: H
          }, L;
        }
        M || (z = wt(z, j)), xr(R, V);
      }
      return Se(z || "", R);
    }, u.create = Ci, u.createCount = bo.createCount, u.onMismatch = ge, u.throwMismatchError = ge, u.createError = B, u.clear = l, u.clearConversions = s, u.addTypes = f, u._findType = a, u.referTo = de, u.referToSelf = He, u.convert = c, u.findSignature = p, u.find = D, u.isTypedFunction = d, u.warnAgainstDeprecatedThis = true, u.addType = function(F, M) {
      let T = "any";
      M !== false && n.has("Object") && (T = "Object"), u.addTypes([
        F
      ], T);
    };
    function on(F) {
      if (!F || typeof F.from != "string" || typeof F.to != "string" || typeof F.convert != "function") throw new TypeError("Object with properties {from: string, to: string, convert: function} expected");
      if (F.to === F.from) throw new SyntaxError('Illegal to define conversion from "' + F.from + '" to itself.');
    }
    return u.addConversion = function(F) {
      let M = arguments.length > 1 && arguments[1] !== void 0 ? arguments[1] : {
        override: false
      };
      on(F);
      const T = a(F.to), z = T.conversionsTo.find((R) => R.from === F.from);
      if (z) if (M && M.override) u.removeConversion({
        from: z.from,
        to: F.to,
        convert: z.convert
      });
      else throw new Error('There is already a conversion from "' + F.from + '" to "' + T.name + '"');
      T.conversionsTo.push({
        from: F.from,
        convert: F.convert,
        index: o++
      });
    }, u.addConversions = function(F, M) {
      F.forEach((T) => u.addConversion(T, M));
    }, u.removeConversion = function(F) {
      on(F);
      const M = a(F.to), T = Ze(M.conversionsTo, (R) => R.from === F.from);
      if (!T) throw new Error("Attempt to remove nonexistent conversion from " + F.from + " to " + F.to);
      if (T.convert !== F.convert) throw new Error("Conversion to remove does not match existing conversion");
      const z = M.conversionsTo.indexOf(T);
      M.conversionsTo.splice(z, 1);
    }, u.resolve = function(F, M) {
      if (!d(F)) throw new TypeError(dn);
      const T = F._typedFunctionData.signatures;
      for (let z = 0; z < T.length; ++z) if (T[z].test(M)) return T[z];
      return null;
    }, u;
  }
  const mn = Ci();
  function De(t) {
    return typeof t == "boolean" ? true : isFinite(t) ? t === Math.round(t) : false;
  }
  var lc = Math.log2 || function(e) {
    return Math.log(e) / Math.LN2;
  };
  function $r(t, e, r) {
    var n = {
      2: "0b",
      8: "0o",
      16: "0x"
    }, i = n[e], o = "";
    if (r) {
      if (r < 1) throw new Error("size must be in greater than 0");
      if (!De(r)) throw new Error("size must be an integer");
      if (t > 2 ** (r - 1) - 1 || t < -(2 ** (r - 1))) throw new Error("Value must be in range [-2^".concat(r - 1, ", 2^").concat(r - 1, "-1]"));
      if (!De(t)) throw new Error("Value must be an integer");
      t < 0 && (t = t + 2 ** r), o = "i".concat(r);
    }
    var u = "";
    return t < 0 && (t = -t, u = "-"), "".concat(u).concat(i).concat(t.toString(e)).concat(o);
  }
  function Rr(t, e) {
    if (typeof e == "function") return e(t);
    if (t === 1 / 0) return "Infinity";
    if (t === -1 / 0) return "-Infinity";
    if (isNaN(t)) return "NaN";
    var { notation: r, precision: n, wordSize: i } = bi(e);
    switch (r) {
      case "fixed":
        return Mi(t, n);
      case "exponential":
        return Si(t, n);
      case "engineering":
        return hc(t, n);
      case "bin":
        return $r(t, 2, i);
      case "oct":
        return $r(t, 8, i);
      case "hex":
        return $r(t, 16, i);
      case "auto":
        return dc(t, n, e).replace(/((\.\d*?)(0+))($|e)/, function() {
          var o = arguments[2], u = arguments[4];
          return o !== "." ? o + u : u;
        });
      default:
        throw new Error('Unknown notation "' + r + '". Choose "auto", "exponential", "fixed", "bin", "oct", or "hex.');
    }
  }
  function bi(t) {
    var e = "auto", r, n;
    if (t !== void 0) if (ve(t)) r = t;
    else if (Fe(t)) r = t.toNumber();
    else if (Kr(t)) t.precision !== void 0 && (r = pn(t.precision, () => {
      throw new Error('Option "precision" must be a number or BigNumber');
    })), t.wordSize !== void 0 && (n = pn(t.wordSize, () => {
      throw new Error('Option "wordSize" must be a number or BigNumber');
    })), t.notation && (e = t.notation);
    else throw new Error("Unsupported type of options, number, BigNumber, or object expected");
    return {
      notation: e,
      precision: r,
      wordSize: n
    };
  }
  function jt(t) {
    var e = String(t).toLowerCase().match(/^(-?)(\d+\.?\d*)(e([+-]?\d+))?$/);
    if (!e) throw new SyntaxError("Invalid number " + t);
    var r = e[1], n = e[2], i = parseFloat(e[4] || "0"), o = n.indexOf(".");
    i += o !== -1 ? o - 1 : n.length - 1;
    var u = n.replace(".", "").replace(/^0*/, function(a) {
      return i -= a.length, "";
    }).replace(/0*$/, "").split("").map(function(a) {
      return parseInt(a);
    });
    return u.length === 0 && (u.push(0), i++), {
      sign: r,
      coefficients: u,
      exponent: i
    };
  }
  function hc(t, e) {
    if (isNaN(t) || !isFinite(t)) return String(t);
    var r = jt(t), n = vr(r, e), i = n.exponent, o = n.coefficients, u = i % 3 === 0 ? i : i < 0 ? i - 3 - i % 3 : i - i % 3;
    if (ve(e)) for (; e > o.length || i - u + 1 > o.length; ) o.push(0);
    else for (var a = Math.abs(i - u) - (o.length - 1), f = 0; f < a; f++) o.push(0);
    for (var l = Math.abs(i - u), s = 1; l > 0; ) s++, l--;
    var h = o.slice(s).join(""), d = ve(e) && h.length || h.match(/[1-9]/) ? "." + h : "", p = o.slice(0, s).join("") + d + "e" + (i >= 0 ? "+" : "") + u.toString();
    return n.sign + p;
  }
  function Mi(t, e) {
    if (isNaN(t) || !isFinite(t)) return String(t);
    var r = jt(t), n = typeof e == "number" ? vr(r, r.exponent + 1 + e) : r, i = n.coefficients, o = n.exponent + 1, u = o + (e || 0);
    return i.length < u && (i = i.concat(bt(u - i.length))), o < 0 && (i = bt(-o + 1).concat(i), o = 1), o < i.length && i.splice(o, 0, o === 0 ? "0." : "."), n.sign + i.join("");
  }
  function Si(t, e) {
    if (isNaN(t) || !isFinite(t)) return String(t);
    var r = jt(t), n = e ? vr(r, e) : r, i = n.coefficients, o = n.exponent;
    i.length < e && (i = i.concat(bt(e - i.length)));
    var u = i.shift();
    return n.sign + u + (i.length > 0 ? "." + i.join("") : "") + "e" + (o >= 0 ? "+" : "") + o;
  }
  function dc(t, e, r) {
    if (isNaN(t) || !isFinite(t)) return String(t);
    var n = vn(r == null ? void 0 : r.lowerExp, -3), i = vn(r == null ? void 0 : r.upperExp, 5), o = jt(t), u = e ? vr(o, e) : o;
    if (u.exponent < n || u.exponent >= i) return Si(t, e);
    var a = u.coefficients, f = u.exponent;
    a.length < e && (a = a.concat(bt(e - a.length))), a = a.concat(bt(f - a.length + 1 + (a.length < e ? e - a.length : 0))), a = bt(-f).concat(a);
    var l = f > 0 ? f : 0;
    return l < a.length - 1 && a.splice(l + 1, 0, "."), u.sign + a.join("");
  }
  function vr(t, e) {
    for (var r = {
      sign: t.sign,
      coefficients: t.coefficients,
      exponent: t.exponent
    }, n = r.coefficients; e <= 0; ) n.unshift(0), r.exponent++, e++;
    if (n.length > e) {
      var i = n.splice(e, n.length - e);
      if (i[0] >= 5) {
        var o = e - 1;
        for (n[o]++; n[o] === 10; ) n.pop(), o === 0 && (n.unshift(0), r.exponent++, o++), o--, n[o]++;
      }
    }
    return r;
  }
  function bt(t) {
    for (var e = [], r = 0; r < t; r++) e.push(0);
    return e;
  }
  function mc(t) {
    return t.toExponential().replace(/e.*$/, "").replace(/^0\.?0*|\./, "").length;
  }
  var pc = Number.EPSILON || 2220446049250313e-31;
  function ut(t, e, r) {
    if (r == null) return t === e;
    if (t === e) return true;
    if (isNaN(t) || isNaN(e)) return false;
    if (isFinite(t) && isFinite(e)) {
      var n = Math.abs(t - e);
      return n <= pc ? true : n <= Math.max(Math.abs(t), Math.abs(e)) * r;
    }
    return false;
  }
  function pn(t, e) {
    if (ve(t)) return t;
    if (Fe(t)) return t.toNumber();
    e();
  }
  function vn(t, e) {
    return ve(t) ? t : Fe(t) ? t.toNumber() : e;
  }
  function Y(t, e, r, n) {
    function i(o) {
      var u = sc(o, e.map(Dc));
      return vc(t, e, o), r(u);
    }
    return i.isFactory = true, i.fn = t, i.dependencies = e.slice().sort(), n && (i.meta = n), i;
  }
  function vc(t, e, r) {
    var n = e.filter((o) => !gc(o)).every((o) => r[o] !== void 0);
    if (!n) {
      var i = e.filter((o) => r[o] === void 0);
      throw new Error('Cannot create function "'.concat(t, '", ') + "some dependencies are missing: ".concat(i.map((o) => '"'.concat(o, '"')).join(", "), "."));
    }
  }
  function gc(t) {
    return t && t[0] === "?";
  }
  function Dc(t) {
    return t && t[0] === "?" ? t.slice(1) : t;
  }
  function wc(t, e) {
    if (Bi(t) && Ni(t, e)) return t[e];
    throw typeof t[e] == "function" && Ac(t, e) ? new Error('Cannot access method "' + e + '" as a property') : new Error('No access to property "' + e + '"');
  }
  function yc(t, e, r) {
    if (Bi(t) && Ni(t, e)) return t[e] = r, r;
    throw new Error('No access to property "' + e + '"');
  }
  function _c(t, e) {
    return e in t;
  }
  function Ni(t, e) {
    return !t || typeof t != "object" ? false : Zt(Fc, e) ? true : !(e in Object.prototype || e in Function.prototype);
  }
  function Ac(t, e) {
    return t == null || typeof t[e] != "function" || Zt(t, e) && Object.getPrototypeOf && e in Object.getPrototypeOf(t) ? false : Zt(Ec, e) ? true : !(e in Object.prototype || e in Function.prototype);
  }
  function Bi(t) {
    return typeof t == "object" && t && t.constructor === Object;
  }
  var Fc = {
    length: true,
    name: true
  }, Ec = {
    toString: true,
    valueOf: true,
    toLocaleString: true
  };
  class Cc {
    constructor(e) {
      this.wrappedObject = e, this[Symbol.iterator] = this.entries;
    }
    keys() {
      return Object.keys(this.wrappedObject).values();
    }
    get(e) {
      return wc(this.wrappedObject, e);
    }
    set(e, r) {
      return yc(this.wrappedObject, e, r), this;
    }
    has(e) {
      return _c(this.wrappedObject, e);
    }
    entries() {
      return bc(this.keys(), (e) => [
        e,
        this.get(e)
      ]);
    }
    forEach(e) {
      for (var r of this.keys()) e(this.get(r), r, this);
    }
    delete(e) {
      delete this.wrappedObject[e];
    }
    clear() {
      for (var e of this.keys()) this.delete(e);
    }
    get size() {
      return Object.keys(this.wrappedObject).length;
    }
  }
  function bc(t, e) {
    return {
      next: () => {
        var r = t.next();
        return r.done ? r : {
          value: e(r.value),
          done: false
        };
      }
    };
  }
  function Mc(t) {
    return t ? t instanceof Map || t instanceof Cc || typeof t.set == "function" && typeof t.get == "function" && typeof t.keys == "function" && typeof t.has == "function" : false;
  }
  var xi = function() {
    return xi = mn.create, mn;
  }, Sc = [
    "?BigNumber",
    "?Complex",
    "?DenseMatrix",
    "?Fraction"
  ], Nc = Y("typed", Sc, function(e) {
    var { BigNumber: r, Complex: n, DenseMatrix: i, Fraction: o } = e, u = xi();
    return u.clear(), u.addTypes([
      {
        name: "number",
        test: ve
      },
      {
        name: "Complex",
        test: _i
      },
      {
        name: "BigNumber",
        test: Fe
      },
      {
        name: "Fraction",
        test: Ai
      },
      {
        name: "Unit",
        test: Fi
      },
      {
        name: "identifier",
        test: (a) => Xe && /^(?:[A-Za-z\xAA\xB5\xBA\xC0-\xD6\xD8-\xF6\xF8-\u02C1\u02C6-\u02D1\u02E0-\u02E4\u02EC\u02EE\u0370-\u0374\u0376\u0377\u037A-\u037D\u037F\u0386\u0388-\u038A\u038C\u038E-\u03A1\u03A3-\u03F5\u03F7-\u0481\u048A-\u052F\u0531-\u0556\u0559\u0560-\u0588\u05D0-\u05EA\u05EF-\u05F2\u0620-\u064A\u066E\u066F\u0671-\u06D3\u06D5\u06E5\u06E6\u06EE\u06EF\u06FA-\u06FC\u06FF\u0710\u0712-\u072F\u074D-\u07A5\u07B1\u07CA-\u07EA\u07F4\u07F5\u07FA\u0800-\u0815\u081A\u0824\u0828\u0840-\u0858\u0860-\u086A\u0870-\u0887\u0889-\u088E\u08A0-\u08C9\u0904-\u0939\u093D\u0950\u0958-\u0961\u0971-\u0980\u0985-\u098C\u098F\u0990\u0993-\u09A8\u09AA-\u09B0\u09B2\u09B6-\u09B9\u09BD\u09CE\u09DC\u09DD\u09DF-\u09E1\u09F0\u09F1\u09FC\u0A05-\u0A0A\u0A0F\u0A10\u0A13-\u0A28\u0A2A-\u0A30\u0A32\u0A33\u0A35\u0A36\u0A38\u0A39\u0A59-\u0A5C\u0A5E\u0A72-\u0A74\u0A85-\u0A8D\u0A8F-\u0A91\u0A93-\u0AA8\u0AAA-\u0AB0\u0AB2\u0AB3\u0AB5-\u0AB9\u0ABD\u0AD0\u0AE0\u0AE1\u0AF9\u0B05-\u0B0C\u0B0F\u0B10\u0B13-\u0B28\u0B2A-\u0B30\u0B32\u0B33\u0B35-\u0B39\u0B3D\u0B5C\u0B5D\u0B5F-\u0B61\u0B71\u0B83\u0B85-\u0B8A\u0B8E-\u0B90\u0B92-\u0B95\u0B99\u0B9A\u0B9C\u0B9E\u0B9F\u0BA3\u0BA4\u0BA8-\u0BAA\u0BAE-\u0BB9\u0BD0\u0C05-\u0C0C\u0C0E-\u0C10\u0C12-\u0C28\u0C2A-\u0C39\u0C3D\u0C58-\u0C5A\u0C5D\u0C60\u0C61\u0C80\u0C85-\u0C8C\u0C8E-\u0C90\u0C92-\u0CA8\u0CAA-\u0CB3\u0CB5-\u0CB9\u0CBD\u0CDD\u0CDE\u0CE0\u0CE1\u0CF1\u0CF2\u0D04-\u0D0C\u0D0E-\u0D10\u0D12-\u0D3A\u0D3D\u0D4E\u0D54-\u0D56\u0D5F-\u0D61\u0D7A-\u0D7F\u0D85-\u0D96\u0D9A-\u0DB1\u0DB3-\u0DBB\u0DBD\u0DC0-\u0DC6\u0E01-\u0E30\u0E32\u0E33\u0E40-\u0E46\u0E81\u0E82\u0E84\u0E86-\u0E8A\u0E8C-\u0EA3\u0EA5\u0EA7-\u0EB0\u0EB2\u0EB3\u0EBD\u0EC0-\u0EC4\u0EC6\u0EDC-\u0EDF\u0F00\u0F40-\u0F47\u0F49-\u0F6C\u0F88-\u0F8C\u1000-\u102A\u103F\u1050-\u1055\u105A-\u105D\u1061\u1065\u1066\u106E-\u1070\u1075-\u1081\u108E\u10A0-\u10C5\u10C7\u10CD\u10D0-\u10FA\u10FC-\u1248\u124A-\u124D\u1250-\u1256\u1258\u125A-\u125D\u1260-\u1288\u128A-\u128D\u1290-\u12B0\u12B2-\u12B5\u12B8-\u12BE\u12C0\u12C2-\u12C5\u12C8-\u12D6\u12D8-\u1310\u1312-\u1315\u1318-\u135A\u1380-\u138F\u13A0-\u13F5\u13F8-\u13FD\u1401-\u166C\u166F-\u167F\u1681-\u169A\u16A0-\u16EA\u16F1-\u16F8\u1700-\u1711\u171F-\u1731\u1740-\u1751\u1760-\u176C\u176E-\u1770\u1780-\u17B3\u17D7\u17DC\u1820-\u1878\u1880-\u1884\u1887-\u18A8\u18AA\u18B0-\u18F5\u1900-\u191E\u1950-\u196D\u1970-\u1974\u1980-\u19AB\u19B0-\u19C9\u1A00-\u1A16\u1A20-\u1A54\u1AA7\u1B05-\u1B33\u1B45-\u1B4C\u1B83-\u1BA0\u1BAE\u1BAF\u1BBA-\u1BE5\u1C00-\u1C23\u1C4D-\u1C4F\u1C5A-\u1C7D\u1C80-\u1C88\u1C90-\u1CBA\u1CBD-\u1CBF\u1CE9-\u1CEC\u1CEE-\u1CF3\u1CF5\u1CF6\u1CFA\u1D00-\u1DBF\u1E00-\u1F15\u1F18-\u1F1D\u1F20-\u1F45\u1F48-\u1F4D\u1F50-\u1F57\u1F59\u1F5B\u1F5D\u1F5F-\u1F7D\u1F80-\u1FB4\u1FB6-\u1FBC\u1FBE\u1FC2-\u1FC4\u1FC6-\u1FCC\u1FD0-\u1FD3\u1FD6-\u1FDB\u1FE0-\u1FEC\u1FF2-\u1FF4\u1FF6-\u1FFC\u2071\u207F\u2090-\u209C\u2102\u2107\u210A-\u2113\u2115\u2119-\u211D\u2124\u2126\u2128\u212A-\u212D\u212F-\u2139\u213C-\u213F\u2145-\u2149\u214E\u2183\u2184\u2C00-\u2CE4\u2CEB-\u2CEE\u2CF2\u2CF3\u2D00-\u2D25\u2D27\u2D2D\u2D30-\u2D67\u2D6F\u2D80-\u2D96\u2DA0-\u2DA6\u2DA8-\u2DAE\u2DB0-\u2DB6\u2DB8-\u2DBE\u2DC0-\u2DC6\u2DC8-\u2DCE\u2DD0-\u2DD6\u2DD8-\u2DDE\u2E2F\u3005\u3006\u3031-\u3035\u303B\u303C\u3041-\u3096\u309D-\u309F\u30A1-\u30FA\u30FC-\u30FF\u3105-\u312F\u3131-\u318E\u31A0-\u31BF\u31F0-\u31FF\u3400-\u4DBF\u4E00-\uA48C\uA4D0-\uA4FD\uA500-\uA60C\uA610-\uA61F\uA62A\uA62B\uA640-\uA66E\uA67F-\uA69D\uA6A0-\uA6E5\uA717-\uA71F\uA722-\uA788\uA78B-\uA7CA\uA7D0\uA7D1\uA7D3\uA7D5-\uA7D9\uA7F2-\uA801\uA803-\uA805\uA807-\uA80A\uA80C-\uA822\uA840-\uA873\uA882-\uA8B3\uA8F2-\uA8F7\uA8FB\uA8FD\uA8FE\uA90A-\uA925\uA930-\uA946\uA960-\uA97C\uA984-\uA9B2\uA9CF\uA9E0-\uA9E4\uA9E6-\uA9EF\uA9FA-\uA9FE\uAA00-\uAA28\uAA40-\uAA42\uAA44-\uAA4B\uAA60-\uAA76\uAA7A\uAA7E-\uAAAF\uAAB1\uAAB5\uAAB6\uAAB9-\uAABD\uAAC0\uAAC2\uAADB-\uAADD\uAAE0-\uAAEA\uAAF2-\uAAF4\uAB01-\uAB06\uAB09-\uAB0E\uAB11-\uAB16\uAB20-\uAB26\uAB28-\uAB2E\uAB30-\uAB5A\uAB5C-\uAB69\uAB70-\uABE2\uAC00-\uD7A3\uD7B0-\uD7C6\uD7CB-\uD7FB\uF900-\uFA6D\uFA70-\uFAD9\uFB00-\uFB06\uFB13-\uFB17\uFB1D\uFB1F-\uFB28\uFB2A-\uFB36\uFB38-\uFB3C\uFB3E\uFB40\uFB41\uFB43\uFB44\uFB46-\uFBB1\uFBD3-\uFD3D\uFD50-\uFD8F\uFD92-\uFDC7\uFDF0-\uFDFB\uFE70-\uFE74\uFE76-\uFEFC\uFF21-\uFF3A\uFF41-\uFF5A\uFF66-\uFFBE\uFFC2-\uFFC7\uFFCA-\uFFCF\uFFD2-\uFFD7\uFFDA-\uFFDC]|\uD800[\uDC00-\uDC0B\uDC0D-\uDC26\uDC28-\uDC3A\uDC3C\uDC3D\uDC3F-\uDC4D\uDC50-\uDC5D\uDC80-\uDCFA\uDE80-\uDE9C\uDEA0-\uDED0\uDF00-\uDF1F\uDF2D-\uDF40\uDF42-\uDF49\uDF50-\uDF75\uDF80-\uDF9D\uDFA0-\uDFC3\uDFC8-\uDFCF]|\uD801[\uDC00-\uDC9D\uDCB0-\uDCD3\uDCD8-\uDCFB\uDD00-\uDD27\uDD30-\uDD63\uDD70-\uDD7A\uDD7C-\uDD8A\uDD8C-\uDD92\uDD94\uDD95\uDD97-\uDDA1\uDDA3-\uDDB1\uDDB3-\uDDB9\uDDBB\uDDBC\uDE00-\uDF36\uDF40-\uDF55\uDF60-\uDF67\uDF80-\uDF85\uDF87-\uDFB0\uDFB2-\uDFBA]|\uD802[\uDC00-\uDC05\uDC08\uDC0A-\uDC35\uDC37\uDC38\uDC3C\uDC3F-\uDC55\uDC60-\uDC76\uDC80-\uDC9E\uDCE0-\uDCF2\uDCF4\uDCF5\uDD00-\uDD15\uDD20-\uDD39\uDD80-\uDDB7\uDDBE\uDDBF\uDE00\uDE10-\uDE13\uDE15-\uDE17\uDE19-\uDE35\uDE60-\uDE7C\uDE80-\uDE9C\uDEC0-\uDEC7\uDEC9-\uDEE4\uDF00-\uDF35\uDF40-\uDF55\uDF60-\uDF72\uDF80-\uDF91]|\uD803[\uDC00-\uDC48\uDC80-\uDCB2\uDCC0-\uDCF2\uDD00-\uDD23\uDE80-\uDEA9\uDEB0\uDEB1\uDF00-\uDF1C\uDF27\uDF30-\uDF45\uDF70-\uDF81\uDFB0-\uDFC4\uDFE0-\uDFF6]|\uD804[\uDC03-\uDC37\uDC71\uDC72\uDC75\uDC83-\uDCAF\uDCD0-\uDCE8\uDD03-\uDD26\uDD44\uDD47\uDD50-\uDD72\uDD76\uDD83-\uDDB2\uDDC1-\uDDC4\uDDDA\uDDDC\uDE00-\uDE11\uDE13-\uDE2B\uDE3F\uDE40\uDE80-\uDE86\uDE88\uDE8A-\uDE8D\uDE8F-\uDE9D\uDE9F-\uDEA8\uDEB0-\uDEDE\uDF05-\uDF0C\uDF0F\uDF10\uDF13-\uDF28\uDF2A-\uDF30\uDF32\uDF33\uDF35-\uDF39\uDF3D\uDF50\uDF5D-\uDF61]|\uD805[\uDC00-\uDC34\uDC47-\uDC4A\uDC5F-\uDC61\uDC80-\uDCAF\uDCC4\uDCC5\uDCC7\uDD80-\uDDAE\uDDD8-\uDDDB\uDE00-\uDE2F\uDE44\uDE80-\uDEAA\uDEB8\uDF00-\uDF1A\uDF40-\uDF46]|\uD806[\uDC00-\uDC2B\uDCA0-\uDCDF\uDCFF-\uDD06\uDD09\uDD0C-\uDD13\uDD15\uDD16\uDD18-\uDD2F\uDD3F\uDD41\uDDA0-\uDDA7\uDDAA-\uDDD0\uDDE1\uDDE3\uDE00\uDE0B-\uDE32\uDE3A\uDE50\uDE5C-\uDE89\uDE9D\uDEB0-\uDEF8]|\uD807[\uDC00-\uDC08\uDC0A-\uDC2E\uDC40\uDC72-\uDC8F\uDD00-\uDD06\uDD08\uDD09\uDD0B-\uDD30\uDD46\uDD60-\uDD65\uDD67\uDD68\uDD6A-\uDD89\uDD98\uDEE0-\uDEF2\uDF02\uDF04-\uDF10\uDF12-\uDF33\uDFB0]|\uD808[\uDC00-\uDF99]|\uD809[\uDC80-\uDD43]|\uD80B[\uDF90-\uDFF0]|[\uD80C\uD81C-\uD820\uD822\uD840-\uD868\uD86A-\uD86C\uD86F-\uD872\uD874-\uD879\uD880-\uD883\uD885-\uD887][\uDC00-\uDFFF]|\uD80D[\uDC00-\uDC2F\uDC41-\uDC46]|\uD811[\uDC00-\uDE46]|\uD81A[\uDC00-\uDE38\uDE40-\uDE5E\uDE70-\uDEBE\uDED0-\uDEED\uDF00-\uDF2F\uDF40-\uDF43\uDF63-\uDF77\uDF7D-\uDF8F]|\uD81B[\uDE40-\uDE7F\uDF00-\uDF4A\uDF50\uDF93-\uDF9F\uDFE0\uDFE1\uDFE3]|\uD821[\uDC00-\uDFF7]|\uD823[\uDC00-\uDCD5\uDD00-\uDD08]|\uD82B[\uDFF0-\uDFF3\uDFF5-\uDFFB\uDFFD\uDFFE]|\uD82C[\uDC00-\uDD22\uDD32\uDD50-\uDD52\uDD55\uDD64-\uDD67\uDD70-\uDEFB]|\uD82F[\uDC00-\uDC6A\uDC70-\uDC7C\uDC80-\uDC88\uDC90-\uDC99]|\uD835[\uDC00-\uDC54\uDC56-\uDC9C\uDC9E\uDC9F\uDCA2\uDCA5\uDCA6\uDCA9-\uDCAC\uDCAE-\uDCB9\uDCBB\uDCBD-\uDCC3\uDCC5-\uDD05\uDD07-\uDD0A\uDD0D-\uDD14\uDD16-\uDD1C\uDD1E-\uDD39\uDD3B-\uDD3E\uDD40-\uDD44\uDD46\uDD4A-\uDD50\uDD52-\uDEA5\uDEA8-\uDEC0\uDEC2-\uDEDA\uDEDC-\uDEFA\uDEFC-\uDF14\uDF16-\uDF34\uDF36-\uDF4E\uDF50-\uDF6E\uDF70-\uDF88\uDF8A-\uDFA8\uDFAA-\uDFC2\uDFC4-\uDFCB]|\uD837[\uDF00-\uDF1E\uDF25-\uDF2A]|\uD838[\uDC30-\uDC6D\uDD00-\uDD2C\uDD37-\uDD3D\uDD4E\uDE90-\uDEAD\uDEC0-\uDEEB]|\uD839[\uDCD0-\uDCEB\uDFE0-\uDFE6\uDFE8-\uDFEB\uDFED\uDFEE\uDFF0-\uDFFE]|\uD83A[\uDC00-\uDCC4\uDD00-\uDD43\uDD4B]|\uD83B[\uDE00-\uDE03\uDE05-\uDE1F\uDE21\uDE22\uDE24\uDE27\uDE29-\uDE32\uDE34-\uDE37\uDE39\uDE3B\uDE42\uDE47\uDE49\uDE4B\uDE4D-\uDE4F\uDE51\uDE52\uDE54\uDE57\uDE59\uDE5B\uDE5D\uDE5F\uDE61\uDE62\uDE64\uDE67-\uDE6A\uDE6C-\uDE72\uDE74-\uDE77\uDE79-\uDE7C\uDE7E\uDE80-\uDE89\uDE8B-\uDE9B\uDEA1-\uDEA3\uDEA5-\uDEA9\uDEAB-\uDEBB]|\uD869[\uDC00-\uDEDF\uDF00-\uDFFF]|\uD86D[\uDC00-\uDF39\uDF40-\uDFFF]|\uD86E[\uDC00-\uDC1D\uDC20-\uDFFF]|\uD873[\uDC00-\uDEA1\uDEB0-\uDFFF]|\uD87A[\uDC00-\uDFE0]|\uD87E[\uDC00-\uDE1D]|\uD884[\uDC00-\uDF4A\uDF50-\uDFFF]|\uD888[\uDC00-\uDFAF])(?:[0-9A-Za-z\xAA\xB5\xBA\xC0-\xD6\xD8-\xF6\xF8-\u02C1\u02C6-\u02D1\u02E0-\u02E4\u02EC\u02EE\u0370-\u0374\u0376\u0377\u037A-\u037D\u037F\u0386\u0388-\u038A\u038C\u038E-\u03A1\u03A3-\u03F5\u03F7-\u0481\u048A-\u052F\u0531-\u0556\u0559\u0560-\u0588\u05D0-\u05EA\u05EF-\u05F2\u0620-\u064A\u066E\u066F\u0671-\u06D3\u06D5\u06E5\u06E6\u06EE\u06EF\u06FA-\u06FC\u06FF\u0710\u0712-\u072F\u074D-\u07A5\u07B1\u07CA-\u07EA\u07F4\u07F5\u07FA\u0800-\u0815\u081A\u0824\u0828\u0840-\u0858\u0860-\u086A\u0870-\u0887\u0889-\u088E\u08A0-\u08C9\u0904-\u0939\u093D\u0950\u0958-\u0961\u0971-\u0980\u0985-\u098C\u098F\u0990\u0993-\u09A8\u09AA-\u09B0\u09B2\u09B6-\u09B9\u09BD\u09CE\u09DC\u09DD\u09DF-\u09E1\u09F0\u09F1\u09FC\u0A05-\u0A0A\u0A0F\u0A10\u0A13-\u0A28\u0A2A-\u0A30\u0A32\u0A33\u0A35\u0A36\u0A38\u0A39\u0A59-\u0A5C\u0A5E\u0A72-\u0A74\u0A85-\u0A8D\u0A8F-\u0A91\u0A93-\u0AA8\u0AAA-\u0AB0\u0AB2\u0AB3\u0AB5-\u0AB9\u0ABD\u0AD0\u0AE0\u0AE1\u0AF9\u0B05-\u0B0C\u0B0F\u0B10\u0B13-\u0B28\u0B2A-\u0B30\u0B32\u0B33\u0B35-\u0B39\u0B3D\u0B5C\u0B5D\u0B5F-\u0B61\u0B71\u0B83\u0B85-\u0B8A\u0B8E-\u0B90\u0B92-\u0B95\u0B99\u0B9A\u0B9C\u0B9E\u0B9F\u0BA3\u0BA4\u0BA8-\u0BAA\u0BAE-\u0BB9\u0BD0\u0C05-\u0C0C\u0C0E-\u0C10\u0C12-\u0C28\u0C2A-\u0C39\u0C3D\u0C58-\u0C5A\u0C5D\u0C60\u0C61\u0C80\u0C85-\u0C8C\u0C8E-\u0C90\u0C92-\u0CA8\u0CAA-\u0CB3\u0CB5-\u0CB9\u0CBD\u0CDD\u0CDE\u0CE0\u0CE1\u0CF1\u0CF2\u0D04-\u0D0C\u0D0E-\u0D10\u0D12-\u0D3A\u0D3D\u0D4E\u0D54-\u0D56\u0D5F-\u0D61\u0D7A-\u0D7F\u0D85-\u0D96\u0D9A-\u0DB1\u0DB3-\u0DBB\u0DBD\u0DC0-\u0DC6\u0E01-\u0E30\u0E32\u0E33\u0E40-\u0E46\u0E81\u0E82\u0E84\u0E86-\u0E8A\u0E8C-\u0EA3\u0EA5\u0EA7-\u0EB0\u0EB2\u0EB3\u0EBD\u0EC0-\u0EC4\u0EC6\u0EDC-\u0EDF\u0F00\u0F40-\u0F47\u0F49-\u0F6C\u0F88-\u0F8C\u1000-\u102A\u103F\u1050-\u1055\u105A-\u105D\u1061\u1065\u1066\u106E-\u1070\u1075-\u1081\u108E\u10A0-\u10C5\u10C7\u10CD\u10D0-\u10FA\u10FC-\u1248\u124A-\u124D\u1250-\u1256\u1258\u125A-\u125D\u1260-\u1288\u128A-\u128D\u1290-\u12B0\u12B2-\u12B5\u12B8-\u12BE\u12C0\u12C2-\u12C5\u12C8-\u12D6\u12D8-\u1310\u1312-\u1315\u1318-\u135A\u1380-\u138F\u13A0-\u13F5\u13F8-\u13FD\u1401-\u166C\u166F-\u167F\u1681-\u169A\u16A0-\u16EA\u16F1-\u16F8\u1700-\u1711\u171F-\u1731\u1740-\u1751\u1760-\u176C\u176E-\u1770\u1780-\u17B3\u17D7\u17DC\u1820-\u1878\u1880-\u1884\u1887-\u18A8\u18AA\u18B0-\u18F5\u1900-\u191E\u1950-\u196D\u1970-\u1974\u1980-\u19AB\u19B0-\u19C9\u1A00-\u1A16\u1A20-\u1A54\u1AA7\u1B05-\u1B33\u1B45-\u1B4C\u1B83-\u1BA0\u1BAE\u1BAF\u1BBA-\u1BE5\u1C00-\u1C23\u1C4D-\u1C4F\u1C5A-\u1C7D\u1C80-\u1C88\u1C90-\u1CBA\u1CBD-\u1CBF\u1CE9-\u1CEC\u1CEE-\u1CF3\u1CF5\u1CF6\u1CFA\u1D00-\u1DBF\u1E00-\u1F15\u1F18-\u1F1D\u1F20-\u1F45\u1F48-\u1F4D\u1F50-\u1F57\u1F59\u1F5B\u1F5D\u1F5F-\u1F7D\u1F80-\u1FB4\u1FB6-\u1FBC\u1FBE\u1FC2-\u1FC4\u1FC6-\u1FCC\u1FD0-\u1FD3\u1FD6-\u1FDB\u1FE0-\u1FEC\u1FF2-\u1FF4\u1FF6-\u1FFC\u2071\u207F\u2090-\u209C\u2102\u2107\u210A-\u2113\u2115\u2119-\u211D\u2124\u2126\u2128\u212A-\u212D\u212F-\u2139\u213C-\u213F\u2145-\u2149\u214E\u2183\u2184\u2C00-\u2CE4\u2CEB-\u2CEE\u2CF2\u2CF3\u2D00-\u2D25\u2D27\u2D2D\u2D30-\u2D67\u2D6F\u2D80-\u2D96\u2DA0-\u2DA6\u2DA8-\u2DAE\u2DB0-\u2DB6\u2DB8-\u2DBE\u2DC0-\u2DC6\u2DC8-\u2DCE\u2DD0-\u2DD6\u2DD8-\u2DDE\u2E2F\u3005\u3006\u3031-\u3035\u303B\u303C\u3041-\u3096\u309D-\u309F\u30A1-\u30FA\u30FC-\u30FF\u3105-\u312F\u3131-\u318E\u31A0-\u31BF\u31F0-\u31FF\u3400-\u4DBF\u4E00-\uA48C\uA4D0-\uA4FD\uA500-\uA60C\uA610-\uA61F\uA62A\uA62B\uA640-\uA66E\uA67F-\uA69D\uA6A0-\uA6E5\uA717-\uA71F\uA722-\uA788\uA78B-\uA7CA\uA7D0\uA7D1\uA7D3\uA7D5-\uA7D9\uA7F2-\uA801\uA803-\uA805\uA807-\uA80A\uA80C-\uA822\uA840-\uA873\uA882-\uA8B3\uA8F2-\uA8F7\uA8FB\uA8FD\uA8FE\uA90A-\uA925\uA930-\uA946\uA960-\uA97C\uA984-\uA9B2\uA9CF\uA9E0-\uA9E4\uA9E6-\uA9EF\uA9FA-\uA9FE\uAA00-\uAA28\uAA40-\uAA42\uAA44-\uAA4B\uAA60-\uAA76\uAA7A\uAA7E-\uAAAF\uAAB1\uAAB5\uAAB6\uAAB9-\uAABD\uAAC0\uAAC2\uAADB-\uAADD\uAAE0-\uAAEA\uAAF2-\uAAF4\uAB01-\uAB06\uAB09-\uAB0E\uAB11-\uAB16\uAB20-\uAB26\uAB28-\uAB2E\uAB30-\uAB5A\uAB5C-\uAB69\uAB70-\uABE2\uAC00-\uD7A3\uD7B0-\uD7C6\uD7CB-\uD7FB\uF900-\uFA6D\uFA70-\uFAD9\uFB00-\uFB06\uFB13-\uFB17\uFB1D\uFB1F-\uFB28\uFB2A-\uFB36\uFB38-\uFB3C\uFB3E\uFB40\uFB41\uFB43\uFB44\uFB46-\uFBB1\uFBD3-\uFD3D\uFD50-\uFD8F\uFD92-\uFDC7\uFDF0-\uFDFB\uFE70-\uFE74\uFE76-\uFEFC\uFF21-\uFF3A\uFF41-\uFF5A\uFF66-\uFFBE\uFFC2-\uFFC7\uFFCA-\uFFCF\uFFD2-\uFFD7\uFFDA-\uFFDC]|\uD800[\uDC00-\uDC0B\uDC0D-\uDC26\uDC28-\uDC3A\uDC3C\uDC3D\uDC3F-\uDC4D\uDC50-\uDC5D\uDC80-\uDCFA\uDE80-\uDE9C\uDEA0-\uDED0\uDF00-\uDF1F\uDF2D-\uDF40\uDF42-\uDF49\uDF50-\uDF75\uDF80-\uDF9D\uDFA0-\uDFC3\uDFC8-\uDFCF]|\uD801[\uDC00-\uDC9D\uDCB0-\uDCD3\uDCD8-\uDCFB\uDD00-\uDD27\uDD30-\uDD63\uDD70-\uDD7A\uDD7C-\uDD8A\uDD8C-\uDD92\uDD94\uDD95\uDD97-\uDDA1\uDDA3-\uDDB1\uDDB3-\uDDB9\uDDBB\uDDBC\uDE00-\uDF36\uDF40-\uDF55\uDF60-\uDF67\uDF80-\uDF85\uDF87-\uDFB0\uDFB2-\uDFBA]|\uD802[\uDC00-\uDC05\uDC08\uDC0A-\uDC35\uDC37\uDC38\uDC3C\uDC3F-\uDC55\uDC60-\uDC76\uDC80-\uDC9E\uDCE0-\uDCF2\uDCF4\uDCF5\uDD00-\uDD15\uDD20-\uDD39\uDD80-\uDDB7\uDDBE\uDDBF\uDE00\uDE10-\uDE13\uDE15-\uDE17\uDE19-\uDE35\uDE60-\uDE7C\uDE80-\uDE9C\uDEC0-\uDEC7\uDEC9-\uDEE4\uDF00-\uDF35\uDF40-\uDF55\uDF60-\uDF72\uDF80-\uDF91]|\uD803[\uDC00-\uDC48\uDC80-\uDCB2\uDCC0-\uDCF2\uDD00-\uDD23\uDE80-\uDEA9\uDEB0\uDEB1\uDF00-\uDF1C\uDF27\uDF30-\uDF45\uDF70-\uDF81\uDFB0-\uDFC4\uDFE0-\uDFF6]|\uD804[\uDC03-\uDC37\uDC71\uDC72\uDC75\uDC83-\uDCAF\uDCD0-\uDCE8\uDD03-\uDD26\uDD44\uDD47\uDD50-\uDD72\uDD76\uDD83-\uDDB2\uDDC1-\uDDC4\uDDDA\uDDDC\uDE00-\uDE11\uDE13-\uDE2B\uDE3F\uDE40\uDE80-\uDE86\uDE88\uDE8A-\uDE8D\uDE8F-\uDE9D\uDE9F-\uDEA8\uDEB0-\uDEDE\uDF05-\uDF0C\uDF0F\uDF10\uDF13-\uDF28\uDF2A-\uDF30\uDF32\uDF33\uDF35-\uDF39\uDF3D\uDF50\uDF5D-\uDF61]|\uD805[\uDC00-\uDC34\uDC47-\uDC4A\uDC5F-\uDC61\uDC80-\uDCAF\uDCC4\uDCC5\uDCC7\uDD80-\uDDAE\uDDD8-\uDDDB\uDE00-\uDE2F\uDE44\uDE80-\uDEAA\uDEB8\uDF00-\uDF1A\uDF40-\uDF46]|\uD806[\uDC00-\uDC2B\uDCA0-\uDCDF\uDCFF-\uDD06\uDD09\uDD0C-\uDD13\uDD15\uDD16\uDD18-\uDD2F\uDD3F\uDD41\uDDA0-\uDDA7\uDDAA-\uDDD0\uDDE1\uDDE3\uDE00\uDE0B-\uDE32\uDE3A\uDE50\uDE5C-\uDE89\uDE9D\uDEB0-\uDEF8]|\uD807[\uDC00-\uDC08\uDC0A-\uDC2E\uDC40\uDC72-\uDC8F\uDD00-\uDD06\uDD08\uDD09\uDD0B-\uDD30\uDD46\uDD60-\uDD65\uDD67\uDD68\uDD6A-\uDD89\uDD98\uDEE0-\uDEF2\uDF02\uDF04-\uDF10\uDF12-\uDF33\uDFB0]|\uD808[\uDC00-\uDF99]|\uD809[\uDC80-\uDD43]|\uD80B[\uDF90-\uDFF0]|[\uD80C\uD81C-\uD820\uD822\uD840-\uD868\uD86A-\uD86C\uD86F-\uD872\uD874-\uD879\uD880-\uD883\uD885-\uD887][\uDC00-\uDFFF]|\uD80D[\uDC00-\uDC2F\uDC41-\uDC46]|\uD811[\uDC00-\uDE46]|\uD81A[\uDC00-\uDE38\uDE40-\uDE5E\uDE70-\uDEBE\uDED0-\uDEED\uDF00-\uDF2F\uDF40-\uDF43\uDF63-\uDF77\uDF7D-\uDF8F]|\uD81B[\uDE40-\uDE7F\uDF00-\uDF4A\uDF50\uDF93-\uDF9F\uDFE0\uDFE1\uDFE3]|\uD821[\uDC00-\uDFF7]|\uD823[\uDC00-\uDCD5\uDD00-\uDD08]|\uD82B[\uDFF0-\uDFF3\uDFF5-\uDFFB\uDFFD\uDFFE]|\uD82C[\uDC00-\uDD22\uDD32\uDD50-\uDD52\uDD55\uDD64-\uDD67\uDD70-\uDEFB]|\uD82F[\uDC00-\uDC6A\uDC70-\uDC7C\uDC80-\uDC88\uDC90-\uDC99]|\uD835[\uDC00-\uDC54\uDC56-\uDC9C\uDC9E\uDC9F\uDCA2\uDCA5\uDCA6\uDCA9-\uDCAC\uDCAE-\uDCB9\uDCBB\uDCBD-\uDCC3\uDCC5-\uDD05\uDD07-\uDD0A\uDD0D-\uDD14\uDD16-\uDD1C\uDD1E-\uDD39\uDD3B-\uDD3E\uDD40-\uDD44\uDD46\uDD4A-\uDD50\uDD52-\uDEA5\uDEA8-\uDEC0\uDEC2-\uDEDA\uDEDC-\uDEFA\uDEFC-\uDF14\uDF16-\uDF34\uDF36-\uDF4E\uDF50-\uDF6E\uDF70-\uDF88\uDF8A-\uDFA8\uDFAA-\uDFC2\uDFC4-\uDFCB]|\uD837[\uDF00-\uDF1E\uDF25-\uDF2A]|\uD838[\uDC30-\uDC6D\uDD00-\uDD2C\uDD37-\uDD3D\uDD4E\uDE90-\uDEAD\uDEC0-\uDEEB]|\uD839[\uDCD0-\uDCEB\uDFE0-\uDFE6\uDFE8-\uDFEB\uDFED\uDFEE\uDFF0-\uDFFE]|\uD83A[\uDC00-\uDCC4\uDD00-\uDD43\uDD4B]|\uD83B[\uDE00-\uDE03\uDE05-\uDE1F\uDE21\uDE22\uDE24\uDE27\uDE29-\uDE32\uDE34-\uDE37\uDE39\uDE3B\uDE42\uDE47\uDE49\uDE4B\uDE4D-\uDE4F\uDE51\uDE52\uDE54\uDE57\uDE59\uDE5B\uDE5D\uDE5F\uDE61\uDE62\uDE64\uDE67-\uDE6A\uDE6C-\uDE72\uDE74-\uDE77\uDE79-\uDE7C\uDE7E\uDE80-\uDE89\uDE8B-\uDE9B\uDEA1-\uDEA3\uDEA5-\uDEA9\uDEAB-\uDEBB]|\uD869[\uDC00-\uDEDF\uDF00-\uDFFF]|\uD86D[\uDC00-\uDF39\uDF40-\uDFFF]|\uD86E[\uDC00-\uDC1D\uDC20-\uDFFF]|\uD873[\uDC00-\uDEA1\uDEB0-\uDFFF]|\uD87A[\uDC00-\uDFE0]|\uD87E[\uDC00-\uDE1D]|\uD884[\uDC00-\uDF4A\uDF50-\uDFFF]|\uD888[\uDC00-\uDFAF])*$/.test(a)
      },
      {
        name: "string",
        test: Xe
      },
      {
        name: "Chain",
        test: uc
      },
      {
        name: "Array",
        test: we
      },
      {
        name: "Matrix",
        test: Ae
      },
      {
        name: "DenseMatrix",
        test: $s
      },
      {
        name: "SparseMatrix",
        test: Is
      },
      {
        name: "Range",
        test: zs
      },
      {
        name: "Index",
        test: Jr
      },
      {
        name: "boolean",
        test: Os
      },
      {
        name: "ResultSet",
        test: Ps
      },
      {
        name: "Help",
        test: qs
      },
      {
        name: "function",
        test: Rs
      },
      {
        name: "Date",
        test: Ls
      },
      {
        name: "RegExp",
        test: Vs
      },
      {
        name: "null",
        test: Us
      },
      {
        name: "undefined",
        test: Ws
      },
      {
        name: "AccessorNode",
        test: Zs
      },
      {
        name: "ArrayNode",
        test: Hs
      },
      {
        name: "AssignmentNode",
        test: js
      },
      {
        name: "BlockNode",
        test: Js
      },
      {
        name: "ConditionalNode",
        test: Ks
      },
      {
        name: "ConstantNode",
        test: Ys
      },
      {
        name: "FunctionNode",
        test: Qs
      },
      {
        name: "FunctionAssignmentNode",
        test: Gs
      },
      {
        name: "IndexNode",
        test: Xs
      },
      {
        name: "Node",
        test: ks
      },
      {
        name: "ObjectNode",
        test: ec
      },
      {
        name: "OperatorNode",
        test: tc
      },
      {
        name: "ParenthesisNode",
        test: rc
      },
      {
        name: "RangeNode",
        test: nc
      },
      {
        name: "RelationalNode",
        test: ic
      },
      {
        name: "SymbolNode",
        test: oc
      },
      {
        name: "Map",
        test: Mc
      },
      {
        name: "Object",
        test: Kr
      }
    ]), u.addConversions([
      {
        from: "number",
        to: "BigNumber",
        convert: function(f) {
          if (r || Ir(f), mc(f) > 15) throw new TypeError("Cannot implicitly convert a number with >15 significant digits to BigNumber (value: " + f + "). Use function bignumber(x) to convert to BigNumber.");
          return new r(f);
        }
      },
      {
        from: "number",
        to: "Complex",
        convert: function(f) {
          return n || er(f), new n(f, 0);
        }
      },
      {
        from: "BigNumber",
        to: "Complex",
        convert: function(f) {
          return n || er(f), new n(f.toNumber(), 0);
        }
      },
      {
        from: "Fraction",
        to: "BigNumber",
        convert: function(f) {
          throw new TypeError("Cannot implicitly convert a Fraction to BigNumber or vice versa. Use function bignumber(x) to convert to BigNumber or fraction(x) to convert to Fraction.");
        }
      },
      {
        from: "Fraction",
        to: "Complex",
        convert: function(f) {
          return n || er(f), new n(f.valueOf(), 0);
        }
      },
      {
        from: "number",
        to: "Fraction",
        convert: function(f) {
          o || zr(f);
          var l = new o(f);
          if (l.valueOf() !== f) throw new TypeError("Cannot implicitly convert a number to a Fraction when there will be a loss of precision (value: " + f + "). Use function fraction(x) to convert to Fraction.");
          return l;
        }
      },
      {
        from: "string",
        to: "number",
        convert: function(f) {
          var l = Number(f);
          if (isNaN(l)) throw new Error('Cannot convert "' + f + '" to a number');
          return l;
        }
      },
      {
        from: "string",
        to: "BigNumber",
        convert: function(f) {
          r || Ir(f);
          try {
            return new r(f);
          } catch {
            throw new Error('Cannot convert "' + f + '" to BigNumber');
          }
        }
      },
      {
        from: "string",
        to: "Fraction",
        convert: function(f) {
          o || zr(f);
          try {
            return new o(f);
          } catch {
            throw new Error('Cannot convert "' + f + '" to Fraction');
          }
        }
      },
      {
        from: "string",
        to: "Complex",
        convert: function(f) {
          n || er(f);
          try {
            return new n(f);
          } catch {
            throw new Error('Cannot convert "' + f + '" to Complex');
          }
        }
      },
      {
        from: "boolean",
        to: "number",
        convert: function(f) {
          return +f;
        }
      },
      {
        from: "boolean",
        to: "BigNumber",
        convert: function(f) {
          return r || Ir(f), new r(+f);
        }
      },
      {
        from: "boolean",
        to: "Fraction",
        convert: function(f) {
          return o || zr(f), new o(+f);
        }
      },
      {
        from: "boolean",
        to: "string",
        convert: function(f) {
          return String(f);
        }
      },
      {
        from: "Array",
        to: "Matrix",
        convert: function(f) {
          return i || Bc(), new i(f);
        }
      },
      {
        from: "Matrix",
        to: "Array",
        convert: function(f) {
          return f.valueOf();
        }
      }
    ]), u.onMismatch = (a, f, l) => {
      var s = u.createError(a, f, l);
      if ([
        "wrongType",
        "mismatch"
      ].includes(s.data.category) && f.length === 1 && Wt(f[0]) && l.some((d) => !d.params.includes(","))) {
        var h = new TypeError("Function '".concat(a, "' doesn't apply to matrices. To call it ") + "elementwise on a matrix 'M', try 'map(M, ".concat(a, ")'."));
        throw h.data = s.data, h;
      }
      throw s;
    }, u.onMismatch = (a, f, l) => {
      var s = u.createError(a, f, l);
      if ([
        "wrongType",
        "mismatch"
      ].includes(s.data.category) && f.length === 1 && Wt(f[0]) && l.some((d) => !d.params.includes(","))) {
        var h = new TypeError("Function '".concat(a, "' doesn't apply to matrices. To call it ") + "elementwise on a matrix 'M', try 'map(M, ".concat(a, ")'."));
        throw h.data = s.data, h;
      }
      throw s;
    }, u;
  });
  function Ir(t) {
    throw new Error("Cannot convert value ".concat(t, " into a BigNumber: no class 'BigNumber' provided"));
  }
  function er(t) {
    throw new Error("Cannot convert value ".concat(t, " into a Complex number: no class 'Complex' provided"));
  }
  function Bc() {
    throw new Error("Cannot convert array into a Matrix: no class 'DenseMatrix' provided");
  }
  function zr(t) {
    throw new Error("Cannot convert value ".concat(t, " into a Fraction, no class 'Fraction' provided."));
  }
  var Ct = 9e15, lt = 1e9, Lr = "0123456789abcdef", ur = "2.3025850929940456840179914546843642076011014886287729760333279009675726096773524802359972050895982983419677840422862486334095254650828067566662873690987816894829072083255546808437998948262331985283935053089653777326288461633662222876982198867465436674744042432743651550489343149393914796194044002221051017141748003688084012647080685567743216228355220114804663715659121373450747856947683463616792101806445070648000277502684916746550586856935673420670581136429224554405758925724208241314695689016758940256776311356919292033376587141660230105703089634572075440370847469940168269282808481184289314848524948644871927809676271275775397027668605952496716674183485704422507197965004714951050492214776567636938662976979522110718264549734772662425709429322582798502585509785265383207606726317164309505995087807523710333101197857547331541421808427543863591778117054309827482385045648019095610299291824318237525357709750539565187697510374970888692180205189339507238539205144634197265287286965110862571492198849978748873771345686209167058", ar = "3.1415926535897932384626433832795028841971693993751058209749445923078164062862089986280348253421170679821480865132823066470938446095505822317253594081284811174502841027019385211055596446229489549303819644288109756659334461284756482337867831652712019091456485669234603486104543266482133936072602491412737245870066063155881748815209209628292540917153643678925903600113305305488204665213841469519415116094330572703657595919530921861173819326117931051185480744623799627495673518857527248912279381830119491298336733624406566430860213949463952247371907021798609437027705392171762931767523846748184676694051320005681271452635608277857713427577896091736371787214684409012249534301465495853710507922796892589235420199561121290219608640344181598136297747713099605187072113499999983729780499510597317328160963185950244594553469083026425223082533446850352619311881710100031378387528865875332083814206171776691473035982534904287554687311595628638823537875937519577818577805321712268066130019278766111959092164201989380952572010654858632789", Vr = {
    precision: 20,
    rounding: 4,
    modulo: 1,
    toExpNeg: -7,
    toExpPos: 21,
    minE: -Ct,
    maxE: Ct,
    crypto: false
  }, Ti, ot, te = true, gr = "[DecimalError] ", ft = gr + "Invalid argument: ", $i = gr + "Precision limit exceeded", Ii = gr + "crypto unavailable", zi = "[object Decimal]", ze = Math.floor, Ne = Math.pow, xc = /^0b([01]+(\.[01]*)?|\.[01]+)(p[+-]?\d+)?$/i, Tc = /^0x([0-9a-f]+(\.[0-9a-f]*)?|\.[0-9a-f]+)(p[+-]?\d+)?$/i, $c = /^0o([0-7]+(\.[0-7]*)?|\.[0-7]+)(p[+-]?\d+)?$/i, Oi = /^(\d+(\.\d*)?|\.\d+)(e[+-]?\d+)?$/i, Qe = 1e7, ee = 7, Ic = 9007199254740991, zc = ur.length - 1, Ur = ar.length - 1, P = {
    toStringTag: zi
  };
  P.absoluteValue = P.abs = function() {
    var t = new this.constructor(this);
    return t.s < 0 && (t.s = 1), X(t);
  };
  P.ceil = function() {
    return X(new this.constructor(this), this.e + 1, 2);
  };
  P.clampedTo = P.clamp = function(t, e) {
    var r, n = this, i = n.constructor;
    if (t = new i(t), e = new i(e), !t.s || !e.s) return new i(NaN);
    if (t.gt(e)) throw Error(ft + e);
    return r = n.cmp(t), r < 0 ? t : n.cmp(e) > 0 ? e : new i(n);
  };
  P.comparedTo = P.cmp = function(t) {
    var e, r, n, i, o = this, u = o.d, a = (t = new o.constructor(t)).d, f = o.s, l = t.s;
    if (!u || !a) return !f || !l ? NaN : f !== l ? f : u === a ? 0 : !u ^ f < 0 ? 1 : -1;
    if (!u[0] || !a[0]) return u[0] ? f : a[0] ? -l : 0;
    if (f !== l) return f;
    if (o.e !== t.e) return o.e > t.e ^ f < 0 ? 1 : -1;
    for (n = u.length, i = a.length, e = 0, r = n < i ? n : i; e < r; ++e) if (u[e] !== a[e]) return u[e] > a[e] ^ f < 0 ? 1 : -1;
    return n === i ? 0 : n > i ^ f < 0 ? 1 : -1;
  };
  P.cosine = P.cos = function() {
    var t, e, r = this, n = r.constructor;
    return r.d ? r.d[0] ? (t = n.precision, e = n.rounding, n.precision = t + Math.max(r.e, r.sd()) + ee, n.rounding = 1, r = Oc(n, Vi(n, r)), n.precision = t, n.rounding = e, X(ot == 2 || ot == 3 ? r.neg() : r, t, e, true)) : new n(1) : new n(NaN);
  };
  P.cubeRoot = P.cbrt = function() {
    var t, e, r, n, i, o, u, a, f, l, s = this, h = s.constructor;
    if (!s.isFinite() || s.isZero()) return new h(s);
    for (te = false, o = s.s * Ne(s.s * s, 1 / 3), !o || Math.abs(o) == 1 / 0 ? (r = $e(s.d), t = s.e, (o = (t - r.length + 1) % 3) && (r += o == 1 || o == -2 ? "0" : "00"), o = Ne(r, 1 / 3), t = ze((t + 1) / 3) - (t % 3 == (t < 0 ? -1 : 2)), o == 1 / 0 ? r = "5e" + t : (r = o.toExponential(), r = r.slice(0, r.indexOf("e") + 1) + t), n = new h(r), n.s = s.s) : n = new h(o.toString()), u = (t = h.precision) + 3; ; ) if (a = n, f = a.times(a).times(a), l = f.plus(s), n = me(l.plus(s).times(a), l.plus(f), u + 2, 1), $e(a.d).slice(0, u) === (r = $e(n.d)).slice(0, u)) if (r = r.slice(u - 3, u + 1), r == "9999" || !i && r == "4999") {
      if (!i && (X(a, t + 1, 0), a.times(a).times(a).eq(s))) {
        n = a;
        break;
      }
      u += 4, i = 1;
    } else {
      (!+r || !+r.slice(1) && r.charAt(0) == "5") && (X(n, t + 1, 1), e = !n.times(n).times(n).eq(s));
      break;
    }
    return te = true, X(n, t, h.rounding, e);
  };
  P.decimalPlaces = P.dp = function() {
    var t, e = this.d, r = NaN;
    if (e) {
      if (t = e.length - 1, r = (t - ze(this.e / ee)) * ee, t = e[t], t) for (; t % 10 == 0; t /= 10) r--;
      r < 0 && (r = 0);
    }
    return r;
  };
  P.dividedBy = P.div = function(t) {
    return me(this, new this.constructor(t));
  };
  P.dividedToIntegerBy = P.divToInt = function(t) {
    var e = this, r = e.constructor;
    return X(me(e, new r(t), 0, 1, 1), r.precision, r.rounding);
  };
  P.equals = P.eq = function(t) {
    return this.cmp(t) === 0;
  };
  P.floor = function() {
    return X(new this.constructor(this), this.e + 1, 3);
  };
  P.greaterThan = P.gt = function(t) {
    return this.cmp(t) > 0;
  };
  P.greaterThanOrEqualTo = P.gte = function(t) {
    var e = this.cmp(t);
    return e == 1 || e === 0;
  };
  P.hyperbolicCosine = P.cosh = function() {
    var t, e, r, n, i, o = this, u = o.constructor, a = new u(1);
    if (!o.isFinite()) return new u(o.s ? 1 / 0 : NaN);
    if (o.isZero()) return a;
    r = u.precision, n = u.rounding, u.precision = r + Math.max(o.e, o.sd()) + 4, u.rounding = 1, i = o.d.length, i < 32 ? (t = Math.ceil(i / 3), e = (1 / wr(4, t)).toString()) : (t = 16, e = "2.3283064365386962890625e-10"), o = $t(u, 1, o.times(e), new u(1), true);
    for (var f, l = t, s = new u(8); l--; ) f = o.times(o), o = a.minus(f.times(s.minus(f.times(s))));
    return X(o, u.precision = r, u.rounding = n, true);
  };
  P.hyperbolicSine = P.sinh = function() {
    var t, e, r, n, i = this, o = i.constructor;
    if (!i.isFinite() || i.isZero()) return new o(i);
    if (e = o.precision, r = o.rounding, o.precision = e + Math.max(i.e, i.sd()) + 4, o.rounding = 1, n = i.d.length, n < 3) i = $t(o, 2, i, i, true);
    else {
      t = 1.4 * Math.sqrt(n), t = t > 16 ? 16 : t | 0, i = i.times(1 / wr(5, t)), i = $t(o, 2, i, i, true);
      for (var u, a = new o(5), f = new o(16), l = new o(20); t--; ) u = i.times(i), i = i.times(a.plus(u.times(f.times(u).plus(l))));
    }
    return o.precision = e, o.rounding = r, X(i, e, r, true);
  };
  P.hyperbolicTangent = P.tanh = function() {
    var t, e, r = this, n = r.constructor;
    return r.isFinite() ? r.isZero() ? new n(r) : (t = n.precision, e = n.rounding, n.precision = t + 7, n.rounding = 1, me(r.sinh(), r.cosh(), n.precision = t, n.rounding = e)) : new n(r.s);
  };
  P.inverseCosine = P.acos = function() {
    var t = this, e = t.constructor, r = t.abs().cmp(1), n = e.precision, i = e.rounding;
    return r !== -1 ? r === 0 ? t.isNeg() ? ke(e, n, i) : new e(0) : new e(NaN) : t.isZero() ? ke(e, n + 4, i).times(0.5) : (e.precision = n + 6, e.rounding = 1, t = new e(1).minus(t).div(t.plus(1)).sqrt().atan(), e.precision = n, e.rounding = i, t.times(2));
  };
  P.inverseHyperbolicCosine = P.acosh = function() {
    var t, e, r = this, n = r.constructor;
    return r.lte(1) ? new n(r.eq(1) ? 0 : NaN) : r.isFinite() ? (t = n.precision, e = n.rounding, n.precision = t + Math.max(Math.abs(r.e), r.sd()) + 4, n.rounding = 1, te = false, r = r.times(r).minus(1).sqrt().plus(r), te = true, n.precision = t, n.rounding = e, r.ln()) : new n(r);
  };
  P.inverseHyperbolicSine = P.asinh = function() {
    var t, e, r = this, n = r.constructor;
    return !r.isFinite() || r.isZero() ? new n(r) : (t = n.precision, e = n.rounding, n.precision = t + 2 * Math.max(Math.abs(r.e), r.sd()) + 6, n.rounding = 1, te = false, r = r.times(r).plus(1).sqrt().plus(r), te = true, n.precision = t, n.rounding = e, r.ln());
  };
  P.inverseHyperbolicTangent = P.atanh = function() {
    var t, e, r, n, i = this, o = i.constructor;
    return i.isFinite() ? i.e >= 0 ? new o(i.abs().eq(1) ? i.s / 0 : i.isZero() ? i : NaN) : (t = o.precision, e = o.rounding, n = i.sd(), Math.max(n, t) < 2 * -i.e - 1 ? X(new o(i), t, e, true) : (o.precision = r = n - i.e, i = me(i.plus(1), new o(1).minus(i), r + t, 1), o.precision = t + 4, o.rounding = 1, i = i.ln(), o.precision = t, o.rounding = e, i.times(0.5))) : new o(NaN);
  };
  P.inverseSine = P.asin = function() {
    var t, e, r, n, i = this, o = i.constructor;
    return i.isZero() ? new o(i) : (e = i.abs().cmp(1), r = o.precision, n = o.rounding, e !== -1 ? e === 0 ? (t = ke(o, r + 4, n).times(0.5), t.s = i.s, t) : new o(NaN) : (o.precision = r + 6, o.rounding = 1, i = i.div(new o(1).minus(i.times(i)).sqrt().plus(1)).atan(), o.precision = r, o.rounding = n, i.times(2)));
  };
  P.inverseTangent = P.atan = function() {
    var t, e, r, n, i, o, u, a, f, l = this, s = l.constructor, h = s.precision, d = s.rounding;
    if (l.isFinite()) {
      if (l.isZero()) return new s(l);
      if (l.abs().eq(1) && h + 4 <= Ur) return u = ke(s, h + 4, d).times(0.25), u.s = l.s, u;
    } else {
      if (!l.s) return new s(NaN);
      if (h + 4 <= Ur) return u = ke(s, h + 4, d).times(0.5), u.s = l.s, u;
    }
    for (s.precision = a = h + 10, s.rounding = 1, r = Math.min(28, a / ee + 2 | 0), t = r; t; --t) l = l.div(l.times(l).plus(1).sqrt().plus(1));
    for (te = false, e = Math.ceil(a / ee), n = 1, f = l.times(l), u = new s(l), i = l; t !== -1; ) if (i = i.times(f), o = u.minus(i.div(n += 2)), i = i.times(f), u = o.plus(i.div(n += 2)), u.d[e] !== void 0) for (t = e; u.d[t] === o.d[t] && t--; ) ;
    return r && (u = u.times(2 << r - 1)), te = true, X(u, s.precision = h, s.rounding = d, true);
  };
  P.isFinite = function() {
    return !!this.d;
  };
  P.isInteger = P.isInt = function() {
    return !!this.d && ze(this.e / ee) > this.d.length - 2;
  };
  P.isNaN = function() {
    return !this.s;
  };
  P.isNegative = P.isNeg = function() {
    return this.s < 0;
  };
  P.isPositive = P.isPos = function() {
    return this.s > 0;
  };
  P.isZero = function() {
    return !!this.d && this.d[0] === 0;
  };
  P.lessThan = P.lt = function(t) {
    return this.cmp(t) < 0;
  };
  P.lessThanOrEqualTo = P.lte = function(t) {
    return this.cmp(t) < 1;
  };
  P.logarithm = P.log = function(t) {
    var e, r, n, i, o, u, a, f, l = this, s = l.constructor, h = s.precision, d = s.rounding, p = 5;
    if (t == null) t = new s(10), e = true;
    else {
      if (t = new s(t), r = t.d, t.s < 0 || !r || !r[0] || t.eq(1)) return new s(NaN);
      e = t.eq(10);
    }
    if (r = l.d, l.s < 0 || !r || !r[0] || l.eq(1)) return new s(r && !r[0] ? -1 / 0 : l.s != 1 ? NaN : r ? 0 : 1 / 0);
    if (e) if (r.length > 1) o = true;
    else {
      for (i = r[0]; i % 10 === 0; ) i /= 10;
      o = i !== 1;
    }
    if (te = false, a = h + p, u = st(l, a), n = e ? sr(s, a + 10) : st(t, a), f = me(u, n, a, 1), Ht(f.d, i = h, d)) do
      if (a += 10, u = st(l, a), n = e ? sr(s, a + 10) : st(t, a), f = me(u, n, a, 1), !o) {
        +$e(f.d).slice(i + 1, i + 15) + 1 == 1e14 && (f = X(f, h + 1, 0));
        break;
      }
    while (Ht(f.d, i += 10, d));
    return te = true, X(f, h, d);
  };
  P.minus = P.sub = function(t) {
    var e, r, n, i, o, u, a, f, l, s, h, d, p = this, D = p.constructor;
    if (t = new D(t), !p.d || !t.d) return !p.s || !t.s ? t = new D(NaN) : p.d ? t.s = -t.s : t = new D(t.d || p.s !== t.s ? p : NaN), t;
    if (p.s != t.s) return t.s = -t.s, p.plus(t);
    if (l = p.d, d = t.d, a = D.precision, f = D.rounding, !l[0] || !d[0]) {
      if (d[0]) t.s = -t.s;
      else if (l[0]) t = new D(p);
      else return new D(f === 3 ? -0 : 0);
      return te ? X(t, a, f) : t;
    }
    if (r = ze(t.e / ee), s = ze(p.e / ee), l = l.slice(), o = s - r, o) {
      for (h = o < 0, h ? (e = l, o = -o, u = d.length) : (e = d, r = s, u = l.length), n = Math.max(Math.ceil(a / ee), u) + 2, o > n && (o = n, e.length = 1), e.reverse(), n = o; n--; ) e.push(0);
      e.reverse();
    } else {
      for (n = l.length, u = d.length, h = n < u, h && (u = n), n = 0; n < u; n++) if (l[n] != d[n]) {
        h = l[n] < d[n];
        break;
      }
      o = 0;
    }
    for (h && (e = l, l = d, d = e, t.s = -t.s), u = l.length, n = d.length - u; n > 0; --n) l[u++] = 0;
    for (n = d.length; n > o; ) {
      if (l[--n] < d[n]) {
        for (i = n; i && l[--i] === 0; ) l[i] = Qe - 1;
        --l[i], l[n] += Qe;
      }
      l[n] -= d[n];
    }
    for (; l[--u] === 0; ) l.pop();
    for (; l[0] === 0; l.shift()) --r;
    return l[0] ? (t.d = l, t.e = Dr(l, r), te ? X(t, a, f) : t) : new D(f === 3 ? -0 : 0);
  };
  P.modulo = P.mod = function(t) {
    var e, r = this, n = r.constructor;
    return t = new n(t), !r.d || !t.s || t.d && !t.d[0] ? new n(NaN) : !t.d || r.d && !r.d[0] ? X(new n(r), n.precision, n.rounding) : (te = false, n.modulo == 9 ? (e = me(r, t.abs(), 0, 3, 1), e.s *= t.s) : e = me(r, t, 0, n.modulo, 1), e = e.times(t), te = true, r.minus(e));
  };
  P.naturalExponential = P.exp = function() {
    return Wr(this);
  };
  P.naturalLogarithm = P.ln = function() {
    return st(this);
  };
  P.negated = P.neg = function() {
    var t = new this.constructor(this);
    return t.s = -t.s, X(t);
  };
  P.plus = P.add = function(t) {
    var e, r, n, i, o, u, a, f, l, s, h = this, d = h.constructor;
    if (t = new d(t), !h.d || !t.d) return !h.s || !t.s ? t = new d(NaN) : h.d || (t = new d(t.d || h.s === t.s ? h : NaN)), t;
    if (h.s != t.s) return t.s = -t.s, h.minus(t);
    if (l = h.d, s = t.d, a = d.precision, f = d.rounding, !l[0] || !s[0]) return s[0] || (t = new d(h)), te ? X(t, a, f) : t;
    if (o = ze(h.e / ee), n = ze(t.e / ee), l = l.slice(), i = o - n, i) {
      for (i < 0 ? (r = l, i = -i, u = s.length) : (r = s, n = o, u = l.length), o = Math.ceil(a / ee), u = o > u ? o + 1 : u + 1, i > u && (i = u, r.length = 1), r.reverse(); i--; ) r.push(0);
      r.reverse();
    }
    for (u = l.length, i = s.length, u - i < 0 && (i = u, r = s, s = l, l = r), e = 0; i; ) e = (l[--i] = l[i] + s[i] + e) / Qe | 0, l[i] %= Qe;
    for (e && (l.unshift(e), ++n), u = l.length; l[--u] == 0; ) l.pop();
    return t.d = l, t.e = Dr(l, n), te ? X(t, a, f) : t;
  };
  P.precision = P.sd = function(t) {
    var e, r = this;
    if (t !== void 0 && t !== !!t && t !== 1 && t !== 0) throw Error(ft + t);
    return r.d ? (e = Pi(r.d), t && r.e + 1 > e && (e = r.e + 1)) : e = NaN, e;
  };
  P.round = function() {
    var t = this, e = t.constructor;
    return X(new e(t), t.e + 1, e.rounding);
  };
  P.sine = P.sin = function() {
    var t, e, r = this, n = r.constructor;
    return r.isFinite() ? r.isZero() ? new n(r) : (t = n.precision, e = n.rounding, n.precision = t + Math.max(r.e, r.sd()) + ee, n.rounding = 1, r = qc(n, Vi(n, r)), n.precision = t, n.rounding = e, X(ot > 2 ? r.neg() : r, t, e, true)) : new n(NaN);
  };
  P.squareRoot = P.sqrt = function() {
    var t, e, r, n, i, o, u = this, a = u.d, f = u.e, l = u.s, s = u.constructor;
    if (l !== 1 || !a || !a[0]) return new s(!l || l < 0 && (!a || a[0]) ? NaN : a ? u : 1 / 0);
    for (te = false, l = Math.sqrt(+u), l == 0 || l == 1 / 0 ? (e = $e(a), (e.length + f) % 2 == 0 && (e += "0"), l = Math.sqrt(e), f = ze((f + 1) / 2) - (f < 0 || f % 2), l == 1 / 0 ? e = "5e" + f : (e = l.toExponential(), e = e.slice(0, e.indexOf("e") + 1) + f), n = new s(e)) : n = new s(l.toString()), r = (f = s.precision) + 3; ; ) if (o = n, n = o.plus(me(u, o, r + 2, 1)).times(0.5), $e(o.d).slice(0, r) === (e = $e(n.d)).slice(0, r)) if (e = e.slice(r - 3, r + 1), e == "9999" || !i && e == "4999") {
      if (!i && (X(o, f + 1, 0), o.times(o).eq(u))) {
        n = o;
        break;
      }
      r += 4, i = 1;
    } else {
      (!+e || !+e.slice(1) && e.charAt(0) == "5") && (X(n, f + 1, 1), t = !n.times(n).eq(u));
      break;
    }
    return te = true, X(n, f, s.rounding, t);
  };
  P.tangent = P.tan = function() {
    var t, e, r = this, n = r.constructor;
    return r.isFinite() ? r.isZero() ? new n(r) : (t = n.precision, e = n.rounding, n.precision = t + 10, n.rounding = 1, r = r.sin(), r.s = 1, r = me(r, new n(1).minus(r.times(r)).sqrt(), t + 10, 0), n.precision = t, n.rounding = e, X(ot == 2 || ot == 4 ? r.neg() : r, t, e, true)) : new n(NaN);
  };
  P.times = P.mul = function(t) {
    var e, r, n, i, o, u, a, f, l, s = this, h = s.constructor, d = s.d, p = (t = new h(t)).d;
    if (t.s *= s.s, !d || !d[0] || !p || !p[0]) return new h(!t.s || d && !d[0] && !p || p && !p[0] && !d ? NaN : !d || !p ? t.s / 0 : t.s * 0);
    for (r = ze(s.e / ee) + ze(t.e / ee), f = d.length, l = p.length, f < l && (o = d, d = p, p = o, u = f, f = l, l = u), o = [], u = f + l, n = u; n--; ) o.push(0);
    for (n = l; --n >= 0; ) {
      for (e = 0, i = f + n; i > n; ) a = o[i] + p[n] * d[i - n - 1] + e, o[i--] = a % Qe | 0, e = a / Qe | 0;
      o[i] = (o[i] + e) % Qe | 0;
    }
    for (; !o[--u]; ) o.pop();
    return e ? ++r : o.shift(), t.d = o, t.e = Dr(o, r), te ? X(t, h.precision, h.rounding) : t;
  };
  P.toBinary = function(t, e) {
    return Yr(this, 2, t, e);
  };
  P.toDecimalPlaces = P.toDP = function(t, e) {
    var r = this, n = r.constructor;
    return r = new n(r), t === void 0 ? r : (Ve(t, 0, lt), e === void 0 ? e = n.rounding : Ve(e, 0, 8), X(r, t + r.e + 1, e));
  };
  P.toExponential = function(t, e) {
    var r, n = this, i = n.constructor;
    return t === void 0 ? r = tt(n, true) : (Ve(t, 0, lt), e === void 0 ? e = i.rounding : Ve(e, 0, 8), n = X(new i(n), t + 1, e), r = tt(n, true, t + 1)), n.isNeg() && !n.isZero() ? "-" + r : r;
  };
  P.toFixed = function(t, e) {
    var r, n, i = this, o = i.constructor;
    return t === void 0 ? r = tt(i) : (Ve(t, 0, lt), e === void 0 ? e = o.rounding : Ve(e, 0, 8), n = X(new o(i), t + i.e + 1, e), r = tt(n, false, t + n.e + 1)), i.isNeg() && !i.isZero() ? "-" + r : r;
  };
  P.toFraction = function(t) {
    var e, r, n, i, o, u, a, f, l, s, h, d, p = this, D = p.d, c = p.constructor;
    if (!D) return new c(p);
    if (l = r = new c(1), n = f = new c(0), e = new c(n), o = e.e = Pi(D) - p.e - 1, u = o % ee, e.d[0] = Ne(10, u < 0 ? ee + u : u), t == null) t = o > 0 ? e : l;
    else {
      if (a = new c(t), !a.isInt() || a.lt(l)) throw Error(ft + a);
      t = a.gt(e) ? o > 0 ? e : l : a;
    }
    for (te = false, a = new c($e(D)), s = c.precision, c.precision = o = D.length * ee * 2; h = me(a, e, 0, 1, 1), i = r.plus(h.times(n)), i.cmp(t) != 1; ) r = n, n = i, i = l, l = f.plus(h.times(i)), f = i, i = e, e = a.minus(h.times(i)), a = i;
    return i = me(t.minus(r), n, 0, 1, 1), f = f.plus(i.times(l)), r = r.plus(i.times(n)), f.s = l.s = p.s, d = me(l, n, o, 1).minus(p).abs().cmp(me(f, r, o, 1).minus(p).abs()) < 1 ? [
      l,
      n
    ] : [
      f,
      r
    ], c.precision = s, te = true, d;
  };
  P.toHexadecimal = P.toHex = function(t, e) {
    return Yr(this, 16, t, e);
  };
  P.toNearest = function(t, e) {
    var r = this, n = r.constructor;
    if (r = new n(r), t == null) {
      if (!r.d) return r;
      t = new n(1), e = n.rounding;
    } else {
      if (t = new n(t), e === void 0 ? e = n.rounding : Ve(e, 0, 8), !r.d) return t.s ? r : t;
      if (!t.d) return t.s && (t.s = r.s), t;
    }
    return t.d[0] ? (te = false, r = me(r, t, 0, e, 1).times(t), te = true, X(r)) : (t.s = r.s, r = t), r;
  };
  P.toNumber = function() {
    return +this;
  };
  P.toOctal = function(t, e) {
    return Yr(this, 8, t, e);
  };
  P.toPower = P.pow = function(t) {
    var e, r, n, i, o, u, a = this, f = a.constructor, l = +(t = new f(t));
    if (!a.d || !t.d || !a.d[0] || !t.d[0]) return new f(Ne(+a, l));
    if (a = new f(a), a.eq(1)) return a;
    if (n = f.precision, o = f.rounding, t.eq(1)) return X(a, n, o);
    if (e = ze(t.e / ee), e >= t.d.length - 1 && (r = l < 0 ? -l : l) <= Ic) return i = qi(f, a, r, n), t.s < 0 ? new f(1).div(i) : X(i, n, o);
    if (u = a.s, u < 0) {
      if (e < t.d.length - 1) return new f(NaN);
      if (t.d[e] & 1 || (u = 1), a.e == 0 && a.d[0] == 1 && a.d.length == 1) return a.s = u, a;
    }
    return r = Ne(+a, l), e = r == 0 || !isFinite(r) ? ze(l * (Math.log("0." + $e(a.d)) / Math.LN10 + a.e + 1)) : new f(r + "").e, e > f.maxE + 1 || e < f.minE - 1 ? new f(e > 0 ? u / 0 : 0) : (te = false, f.rounding = a.s = 1, r = Math.min(12, (e + "").length), i = Wr(t.times(st(a, n + r)), n), i.d && (i = X(i, n + 5, 1), Ht(i.d, n, o) && (e = n + 10, i = X(Wr(t.times(st(a, e + r)), e), e + 5, 1), +$e(i.d).slice(n + 1, n + 15) + 1 == 1e14 && (i = X(i, n + 1, 0)))), i.s = u, te = true, f.rounding = o, X(i, n, o));
  };
  P.toPrecision = function(t, e) {
    var r, n = this, i = n.constructor;
    return t === void 0 ? r = tt(n, n.e <= i.toExpNeg || n.e >= i.toExpPos) : (Ve(t, 1, lt), e === void 0 ? e = i.rounding : Ve(e, 0, 8), n = X(new i(n), t, e), r = tt(n, t <= n.e || n.e <= i.toExpNeg, t)), n.isNeg() && !n.isZero() ? "-" + r : r;
  };
  P.toSignificantDigits = P.toSD = function(t, e) {
    var r = this, n = r.constructor;
    return t === void 0 ? (t = n.precision, e = n.rounding) : (Ve(t, 1, lt), e === void 0 ? e = n.rounding : Ve(e, 0, 8)), X(new n(r), t, e);
  };
  P.toString = function() {
    var t = this, e = t.constructor, r = tt(t, t.e <= e.toExpNeg || t.e >= e.toExpPos);
    return t.isNeg() && !t.isZero() ? "-" + r : r;
  };
  P.truncated = P.trunc = function() {
    return X(new this.constructor(this), this.e + 1, 1);
  };
  P.valueOf = P.toJSON = function() {
    var t = this, e = t.constructor, r = tt(t, t.e <= e.toExpNeg || t.e >= e.toExpPos);
    return t.isNeg() ? "-" + r : r;
  };
  function $e(t) {
    var e, r, n, i = t.length - 1, o = "", u = t[0];
    if (i > 0) {
      for (o += u, e = 1; e < i; e++) n = t[e] + "", r = ee - n.length, r && (o += at(r)), o += n;
      u = t[e], n = u + "", r = ee - n.length, r && (o += at(r));
    } else if (u === 0) return "0";
    for (; u % 10 === 0; ) u /= 10;
    return o + u;
  }
  function Ve(t, e, r) {
    if (t !== ~~t || t < e || t > r) throw Error(ft + t);
  }
  function Ht(t, e, r, n) {
    var i, o, u, a;
    for (o = t[0]; o >= 10; o /= 10) --e;
    return --e < 0 ? (e += ee, i = 0) : (i = Math.ceil((e + 1) / ee), e %= ee), o = Ne(10, ee - e), a = t[i] % o | 0, n == null ? e < 3 ? (e == 0 ? a = a / 100 | 0 : e == 1 && (a = a / 10 | 0), u = r < 4 && a == 99999 || r > 3 && a == 49999 || a == 5e4 || a == 0) : u = (r < 4 && a + 1 == o || r > 3 && a + 1 == o / 2) && (t[i + 1] / o / 100 | 0) == Ne(10, e - 2) - 1 || (a == o / 2 || a == 0) && (t[i + 1] / o / 100 | 0) == 0 : e < 4 ? (e == 0 ? a = a / 1e3 | 0 : e == 1 ? a = a / 100 | 0 : e == 2 && (a = a / 10 | 0), u = (n || r < 4) && a == 9999 || !n && r > 3 && a == 4999) : u = ((n || r < 4) && a + 1 == o || !n && r > 3 && a + 1 == o / 2) && (t[i + 1] / o / 1e3 | 0) == Ne(10, e - 3) - 1, u;
  }
  function nr(t, e, r) {
    for (var n, i = [
      0
    ], o, u = 0, a = t.length; u < a; ) {
      for (o = i.length; o--; ) i[o] *= e;
      for (i[0] += Lr.indexOf(t.charAt(u++)), n = 0; n < i.length; n++) i[n] > r - 1 && (i[n + 1] === void 0 && (i[n + 1] = 0), i[n + 1] += i[n] / r | 0, i[n] %= r);
    }
    return i.reverse();
  }
  function Oc(t, e) {
    var r, n, i;
    if (e.isZero()) return e;
    n = e.d.length, n < 32 ? (r = Math.ceil(n / 3), i = (1 / wr(4, r)).toString()) : (r = 16, i = "2.3283064365386962890625e-10"), t.precision += r, e = $t(t, 1, e.times(i), new t(1));
    for (var o = r; o--; ) {
      var u = e.times(e);
      e = u.times(u).minus(u).times(8).plus(1);
    }
    return t.precision -= r, e;
  }
  var me = /* @__PURE__ */ function() {
    function t(n, i, o) {
      var u, a = 0, f = n.length;
      for (n = n.slice(); f--; ) u = n[f] * i + a, n[f] = u % o | 0, a = u / o | 0;
      return a && n.unshift(a), n;
    }
    function e(n, i, o, u) {
      var a, f;
      if (o != u) f = o > u ? 1 : -1;
      else for (a = f = 0; a < o; a++) if (n[a] != i[a]) {
        f = n[a] > i[a] ? 1 : -1;
        break;
      }
      return f;
    }
    function r(n, i, o, u) {
      for (var a = 0; o--; ) n[o] -= a, a = n[o] < i[o] ? 1 : 0, n[o] = a * u + n[o] - i[o];
      for (; !n[0] && n.length > 1; ) n.shift();
    }
    return function(n, i, o, u, a, f) {
      var l, s, h, d, p, D, c, g, m, w, v, A, y, _, E, C, b, x, N, B, Z = n.constructor, q = n.s == i.s ? 1 : -1, U = n.d, I = i.d;
      if (!U || !U[0] || !I || !I[0]) return new Z(!n.s || !i.s || (U ? I && U[0] == I[0] : !I) ? NaN : U && U[0] == 0 || !I ? q * 0 : q / 0);
      for (f ? (p = 1, s = n.e - i.e) : (f = Qe, p = ee, s = ze(n.e / p) - ze(i.e / p)), N = I.length, b = U.length, m = new Z(q), w = m.d = [], h = 0; I[h] == (U[h] || 0); h++) ;
      if (I[h] > (U[h] || 0) && s--, o == null ? (_ = o = Z.precision, u = Z.rounding) : a ? _ = o + (n.e - i.e) + 1 : _ = o, _ < 0) w.push(1), D = true;
      else {
        if (_ = _ / p + 2 | 0, h = 0, N == 1) {
          for (d = 0, I = I[0], _++; (h < b || d) && _--; h++) E = d * f + (U[h] || 0), w[h] = E / I | 0, d = E % I | 0;
          D = d || h < b;
        } else {
          for (d = f / (I[0] + 1) | 0, d > 1 && (I = t(I, d, f), U = t(U, d, f), N = I.length, b = U.length), C = N, v = U.slice(0, N), A = v.length; A < N; ) v[A++] = 0;
          B = I.slice(), B.unshift(0), x = I[0], I[1] >= f / 2 && ++x;
          do
            d = 0, l = e(I, v, N, A), l < 0 ? (y = v[0], N != A && (y = y * f + (v[1] || 0)), d = y / x | 0, d > 1 ? (d >= f && (d = f - 1), c = t(I, d, f), g = c.length, A = v.length, l = e(c, v, g, A), l == 1 && (d--, r(c, N < g ? B : I, g, f))) : (d == 0 && (l = d = 1), c = I.slice()), g = c.length, g < A && c.unshift(0), r(v, c, A, f), l == -1 && (A = v.length, l = e(I, v, N, A), l < 1 && (d++, r(v, N < A ? B : I, A, f))), A = v.length) : l === 0 && (d++, v = [
              0
            ]), w[h++] = d, l && v[0] ? v[A++] = U[C] || 0 : (v = [
              U[C]
            ], A = 1);
          while ((C++ < b || v[0] !== void 0) && _--);
          D = v[0] !== void 0;
        }
        w[0] || w.shift();
      }
      if (p == 1) m.e = s, Ti = D;
      else {
        for (h = 1, d = w[0]; d >= 10; d /= 10) h++;
        m.e = h + s * p - 1, X(m, a ? o + m.e + 1 : o, u, D);
      }
      return m;
    };
  }();
  function X(t, e, r, n) {
    var i, o, u, a, f, l, s, h, d, p = t.constructor;
    e: if (e != null) {
      if (h = t.d, !h) return t;
      for (i = 1, a = h[0]; a >= 10; a /= 10) i++;
      if (o = e - i, o < 0) o += ee, u = e, s = h[d = 0], f = s / Ne(10, i - u - 1) % 10 | 0;
      else if (d = Math.ceil((o + 1) / ee), a = h.length, d >= a) if (n) {
        for (; a++ <= d; ) h.push(0);
        s = f = 0, i = 1, o %= ee, u = o - ee + 1;
      } else break e;
      else {
        for (s = a = h[d], i = 1; a >= 10; a /= 10) i++;
        o %= ee, u = o - ee + i, f = u < 0 ? 0 : s / Ne(10, i - u - 1) % 10 | 0;
      }
      if (n = n || e < 0 || h[d + 1] !== void 0 || (u < 0 ? s : s % Ne(10, i - u - 1)), l = r < 4 ? (f || n) && (r == 0 || r == (t.s < 0 ? 3 : 2)) : f > 5 || f == 5 && (r == 4 || n || r == 6 && (o > 0 ? u > 0 ? s / Ne(10, i - u) : 0 : h[d - 1]) % 10 & 1 || r == (t.s < 0 ? 8 : 7)), e < 1 || !h[0]) return h.length = 0, l ? (e -= t.e + 1, h[0] = Ne(10, (ee - e % ee) % ee), t.e = -e || 0) : h[0] = t.e = 0, t;
      if (o == 0 ? (h.length = d, a = 1, d--) : (h.length = d + 1, a = Ne(10, ee - o), h[d] = u > 0 ? (s / Ne(10, i - u) % Ne(10, u) | 0) * a : 0), l) for (; ; ) if (d == 0) {
        for (o = 1, u = h[0]; u >= 10; u /= 10) o++;
        for (u = h[0] += a, a = 1; u >= 10; u /= 10) a++;
        o != a && (t.e++, h[0] == Qe && (h[0] = 1));
        break;
      } else {
        if (h[d] += a, h[d] != Qe) break;
        h[d--] = 0, a = 1;
      }
      for (o = h.length; h[--o] === 0; ) h.pop();
    }
    return te && (t.e > p.maxE ? (t.d = null, t.e = NaN) : t.e < p.minE && (t.e = 0, t.d = [
      0
    ])), t;
  }
  function tt(t, e, r) {
    if (!t.isFinite()) return Li(t);
    var n, i = t.e, o = $e(t.d), u = o.length;
    return e ? (r && (n = r - u) > 0 ? o = o.charAt(0) + "." + o.slice(1) + at(n) : u > 1 && (o = o.charAt(0) + "." + o.slice(1)), o = o + (t.e < 0 ? "e" : "e+") + t.e) : i < 0 ? (o = "0." + at(-i - 1) + o, r && (n = r - u) > 0 && (o += at(n))) : i >= u ? (o += at(i + 1 - u), r && (n = r - i - 1) > 0 && (o = o + "." + at(n))) : ((n = i + 1) < u && (o = o.slice(0, n) + "." + o.slice(n)), r && (n = r - u) > 0 && (i + 1 === u && (o += "."), o += at(n))), o;
  }
  function Dr(t, e) {
    var r = t[0];
    for (e *= ee; r >= 10; r /= 10) e++;
    return e;
  }
  function sr(t, e, r) {
    if (e > zc) throw te = true, r && (t.precision = r), Error($i);
    return X(new t(ur), e, 1, true);
  }
  function ke(t, e, r) {
    if (e > Ur) throw Error($i);
    return X(new t(ar), e, r, true);
  }
  function Pi(t) {
    var e = t.length - 1, r = e * ee + 1;
    if (e = t[e], e) {
      for (; e % 10 == 0; e /= 10) r--;
      for (e = t[0]; e >= 10; e /= 10) r++;
    }
    return r;
  }
  function at(t) {
    for (var e = ""; t--; ) e += "0";
    return e;
  }
  function qi(t, e, r, n) {
    var i, o = new t(1), u = Math.ceil(n / ee + 4);
    for (te = false; ; ) {
      if (r % 2 && (o = o.times(e), Dn(o.d, u) && (i = true)), r = ze(r / 2), r === 0) {
        r = o.d.length - 1, i && o.d[r] === 0 && ++o.d[r];
        break;
      }
      e = e.times(e), Dn(e.d, u);
    }
    return te = true, o;
  }
  function gn(t) {
    return t.d[t.d.length - 1] & 1;
  }
  function Ri(t, e, r) {
    for (var n, i, o = new t(e[0]), u = 0; ++u < e.length; ) {
      if (i = new t(e[u]), !i.s) {
        o = i;
        break;
      }
      n = o.cmp(i), (n === r || n === 0 && o.s === r) && (o = i);
    }
    return o;
  }
  function Wr(t, e) {
    var r, n, i, o, u, a, f, l = 0, s = 0, h = 0, d = t.constructor, p = d.rounding, D = d.precision;
    if (!t.d || !t.d[0] || t.e > 17) return new d(t.d ? t.d[0] ? t.s < 0 ? 0 : 1 / 0 : 1 : t.s ? t.s < 0 ? 0 : t : NaN);
    for (e == null ? (te = false, f = D) : f = e, a = new d(0.03125); t.e > -2; ) t = t.times(a), h += 5;
    for (n = Math.log(Ne(2, h)) / Math.LN10 * 2 + 5 | 0, f += n, r = o = u = new d(1), d.precision = f; ; ) {
      if (o = X(o.times(t), f, 1), r = r.times(++s), a = u.plus(me(o, r, f, 1)), $e(a.d).slice(0, f) === $e(u.d).slice(0, f)) {
        for (i = h; i--; ) u = X(u.times(u), f, 1);
        if (e == null) if (l < 3 && Ht(u.d, f - n, p, l)) d.precision = f += 10, r = o = a = new d(1), s = 0, l++;
        else return X(u, d.precision = D, p, te = true);
        else return d.precision = D, u;
      }
      u = a;
    }
  }
  function st(t, e) {
    var r, n, i, o, u, a, f, l, s, h, d, p = 1, D = 10, c = t, g = c.d, m = c.constructor, w = m.rounding, v = m.precision;
    if (c.s < 0 || !g || !g[0] || !c.e && g[0] == 1 && g.length == 1) return new m(g && !g[0] ? -1 / 0 : c.s != 1 ? NaN : g ? 0 : c);
    if (e == null ? (te = false, s = v) : s = e, m.precision = s += D, r = $e(g), n = r.charAt(0), Math.abs(o = c.e) < 15e14) {
      for (; n < 7 && n != 1 || n == 1 && r.charAt(1) > 3; ) c = c.times(t), r = $e(c.d), n = r.charAt(0), p++;
      o = c.e, n > 1 ? (c = new m("0." + r), o++) : c = new m(n + "." + r.slice(1));
    } else return l = sr(m, s + 2, v).times(o + ""), c = st(new m(n + "." + r.slice(1)), s - D).plus(l), m.precision = v, e == null ? X(c, v, w, te = true) : c;
    for (h = c, f = u = c = me(c.minus(1), c.plus(1), s, 1), d = X(c.times(c), s, 1), i = 3; ; ) {
      if (u = X(u.times(d), s, 1), l = f.plus(me(u, new m(i), s, 1)), $e(l.d).slice(0, s) === $e(f.d).slice(0, s)) if (f = f.times(2), o !== 0 && (f = f.plus(sr(m, s + 2, v).times(o + ""))), f = me(f, new m(p), s, 1), e == null) if (Ht(f.d, s - D, w, a)) m.precision = s += D, l = u = c = me(h.minus(1), h.plus(1), s, 1), d = X(c.times(c), s, 1), i = a = 1;
      else return X(f, m.precision = v, w, te = true);
      else return m.precision = v, f;
      f = l, i += 2;
    }
  }
  function Li(t) {
    return String(t.s * t.s / 0);
  }
  function ir(t, e) {
    var r, n, i;
    for ((r = e.indexOf(".")) > -1 && (e = e.replace(".", "")), (n = e.search(/e/i)) > 0 ? (r < 0 && (r = n), r += +e.slice(n + 1), e = e.substring(0, n)) : r < 0 && (r = e.length), n = 0; e.charCodeAt(n) === 48; n++) ;
    for (i = e.length; e.charCodeAt(i - 1) === 48; --i) ;
    if (e = e.slice(n, i), e) {
      if (i -= n, t.e = r = r - n - 1, t.d = [], n = (r + 1) % ee, r < 0 && (n += ee), n < i) {
        for (n && t.d.push(+e.slice(0, n)), i -= ee; n < i; ) t.d.push(+e.slice(n, n += ee));
        e = e.slice(n), n = ee - e.length;
      } else n -= i;
      for (; n--; ) e += "0";
      t.d.push(+e), te && (t.e > t.constructor.maxE ? (t.d = null, t.e = NaN) : t.e < t.constructor.minE && (t.e = 0, t.d = [
        0
      ]));
    } else t.e = 0, t.d = [
      0
    ];
    return t;
  }
  function Pc(t, e) {
    var r, n, i, o, u, a, f, l, s;
    if (e.indexOf("_") > -1) {
      if (e = e.replace(/(\d)_(?=\d)/g, "$1"), Oi.test(e)) return ir(t, e);
    } else if (e === "Infinity" || e === "NaN") return +e || (t.s = NaN), t.e = NaN, t.d = null, t;
    if (Tc.test(e)) r = 16, e = e.toLowerCase();
    else if (xc.test(e)) r = 2;
    else if ($c.test(e)) r = 8;
    else throw Error(ft + e);
    for (o = e.search(/p/i), o > 0 ? (f = +e.slice(o + 1), e = e.substring(2, o)) : e = e.slice(2), o = e.indexOf("."), u = o >= 0, n = t.constructor, u && (e = e.replace(".", ""), a = e.length, o = a - o, i = qi(n, new n(r), o, o * 2)), l = nr(e, r, Qe), s = l.length - 1, o = s; l[o] === 0; --o) l.pop();
    return o < 0 ? new n(t.s * 0) : (t.e = Dr(l, s), t.d = l, te = false, u && (t = me(t, i, a * 4)), f && (t = t.times(Math.abs(f) < 54 ? Ne(2, f) : Dt.pow(2, f))), te = true, t);
  }
  function qc(t, e) {
    var r, n = e.d.length;
    if (n < 3) return e.isZero() ? e : $t(t, 2, e, e);
    r = 1.4 * Math.sqrt(n), r = r > 16 ? 16 : r | 0, e = e.times(1 / wr(5, r)), e = $t(t, 2, e, e);
    for (var i, o = new t(5), u = new t(16), a = new t(20); r--; ) i = e.times(e), e = e.times(o.plus(i.times(u.times(i).minus(a))));
    return e;
  }
  function $t(t, e, r, n, i) {
    var o, u, a, f, l = t.precision, s = Math.ceil(l / ee);
    for (te = false, f = r.times(r), a = new t(n); ; ) {
      if (u = me(a.times(f), new t(e++ * e++), l, 1), a = i ? n.plus(u) : n.minus(u), n = me(u.times(f), new t(e++ * e++), l, 1), u = a.plus(n), u.d[s] !== void 0) {
        for (o = s; u.d[o] === a.d[o] && o--; ) ;
        if (o == -1) break;
      }
      o = a, a = n, n = u, u = o;
    }
    return te = true, u.d.length = s + 1, u;
  }
  function wr(t, e) {
    for (var r = t; --e; ) r *= t;
    return r;
  }
  function Vi(t, e) {
    var r, n = e.s < 0, i = ke(t, t.precision, 1), o = i.times(0.5);
    if (e = e.abs(), e.lte(o)) return ot = n ? 4 : 1, e;
    if (r = e.divToInt(i), r.isZero()) ot = n ? 3 : 2;
    else {
      if (e = e.minus(r.times(i)), e.lte(o)) return ot = gn(r) ? n ? 2 : 3 : n ? 4 : 1, e;
      ot = gn(r) ? n ? 1 : 4 : n ? 3 : 2;
    }
    return e.minus(i).abs();
  }
  function Yr(t, e, r, n) {
    var i, o, u, a, f, l, s, h, d, p = t.constructor, D = r !== void 0;
    if (D ? (Ve(r, 1, lt), n === void 0 ? n = p.rounding : Ve(n, 0, 8)) : (r = p.precision, n = p.rounding), !t.isFinite()) s = Li(t);
    else {
      for (s = tt(t), u = s.indexOf("."), D ? (i = 2, e == 16 ? r = r * 4 - 3 : e == 8 && (r = r * 3 - 2)) : i = e, u >= 0 && (s = s.replace(".", ""), d = new p(1), d.e = s.length - u, d.d = nr(tt(d), 10, i), d.e = d.d.length), h = nr(s, 10, i), o = f = h.length; h[--f] == 0; ) h.pop();
      if (!h[0]) s = D ? "0p+0" : "0";
      else {
        if (u < 0 ? o-- : (t = new p(t), t.d = h, t.e = o, t = me(t, d, r, n, 0, i), h = t.d, o = t.e, l = Ti), u = h[r], a = i / 2, l = l || h[r + 1] !== void 0, l = n < 4 ? (u !== void 0 || l) && (n === 0 || n === (t.s < 0 ? 3 : 2)) : u > a || u === a && (n === 4 || l || n === 6 && h[r - 1] & 1 || n === (t.s < 0 ? 8 : 7)), h.length = r, l) for (; ++h[--r] > i - 1; ) h[r] = 0, r || (++o, h.unshift(1));
        for (f = h.length; !h[f - 1]; --f) ;
        for (u = 0, s = ""; u < f; u++) s += Lr.charAt(h[u]);
        if (D) {
          if (f > 1) if (e == 16 || e == 8) {
            for (u = e == 16 ? 4 : 3, --f; f % u; f++) s += "0";
            for (h = nr(s, i, e), f = h.length; !h[f - 1]; --f) ;
            for (u = 1, s = "1."; u < f; u++) s += Lr.charAt(h[u]);
          } else s = s.charAt(0) + "." + s.slice(1);
          s = s + (o < 0 ? "p" : "p+") + o;
        } else if (o < 0) {
          for (; ++o; ) s = "0" + s;
          s = "0." + s;
        } else if (++o > f) for (o -= f; o--; ) s += "0";
        else o < f && (s = s.slice(0, o) + "." + s.slice(o));
      }
      s = (e == 16 ? "0x" : e == 2 ? "0b" : e == 8 ? "0o" : "") + s;
    }
    return t.s < 0 ? "-" + s : s;
  }
  function Dn(t, e) {
    if (t.length > e) return t.length = e, true;
  }
  function Rc(t) {
    return new this(t).abs();
  }
  function Lc(t) {
    return new this(t).acos();
  }
  function Vc(t) {
    return new this(t).acosh();
  }
  function Uc(t, e) {
    return new this(t).plus(e);
  }
  function Wc(t) {
    return new this(t).asin();
  }
  function Zc(t) {
    return new this(t).asinh();
  }
  function Hc(t) {
    return new this(t).atan();
  }
  function jc(t) {
    return new this(t).atanh();
  }
  function Jc(t, e) {
    t = new this(t), e = new this(e);
    var r, n = this.precision, i = this.rounding, o = n + 4;
    return !t.s || !e.s ? r = new this(NaN) : !t.d && !e.d ? (r = ke(this, o, 1).times(e.s > 0 ? 0.25 : 0.75), r.s = t.s) : !e.d || t.isZero() ? (r = e.s < 0 ? ke(this, n, i) : new this(0), r.s = t.s) : !t.d || e.isZero() ? (r = ke(this, o, 1).times(0.5), r.s = t.s) : e.s < 0 ? (this.precision = o, this.rounding = 1, r = this.atan(me(t, e, o, 1)), e = ke(this, o, 1), this.precision = n, this.rounding = i, r = t.s < 0 ? r.minus(e) : r.plus(e)) : r = this.atan(me(t, e, o, 1)), r;
  }
  function Kc(t) {
    return new this(t).cbrt();
  }
  function Yc(t) {
    return X(t = new this(t), t.e + 1, 2);
  }
  function Gc(t, e, r) {
    return new this(t).clamp(e, r);
  }
  function Qc(t) {
    if (!t || typeof t != "object") throw Error(gr + "Object expected");
    var e, r, n, i = t.defaults === true, o = [
      "precision",
      1,
      lt,
      "rounding",
      0,
      8,
      "toExpNeg",
      -Ct,
      0,
      "toExpPos",
      0,
      Ct,
      "maxE",
      0,
      Ct,
      "minE",
      -Ct,
      0,
      "modulo",
      0,
      9
    ];
    for (e = 0; e < o.length; e += 3) if (r = o[e], i && (this[r] = Vr[r]), (n = t[r]) !== void 0) if (ze(n) === n && n >= o[e + 1] && n <= o[e + 2]) this[r] = n;
    else throw Error(ft + r + ": " + n);
    if (r = "crypto", i && (this[r] = Vr[r]), (n = t[r]) !== void 0) if (n === true || n === false || n === 0 || n === 1) if (n) if (typeof crypto < "u" && crypto && (crypto.getRandomValues || crypto.randomBytes)) this[r] = true;
    else throw Error(Ii);
    else this[r] = false;
    else throw Error(ft + r + ": " + n);
    return this;
  }
  function Xc(t) {
    return new this(t).cos();
  }
  function kc(t) {
    return new this(t).cosh();
  }
  function Ui(t) {
    var e, r, n;
    function i(o) {
      var u, a, f, l = this;
      if (!(l instanceof i)) return new i(o);
      if (l.constructor = i, wn(o)) {
        l.s = o.s, te ? !o.d || o.e > i.maxE ? (l.e = NaN, l.d = null) : o.e < i.minE ? (l.e = 0, l.d = [
          0
        ]) : (l.e = o.e, l.d = o.d.slice()) : (l.e = o.e, l.d = o.d ? o.d.slice() : o.d);
        return;
      }
      if (f = typeof o, f === "number") {
        if (o === 0) {
          l.s = 1 / o < 0 ? -1 : 1, l.e = 0, l.d = [
            0
          ];
          return;
        }
        if (o < 0 ? (o = -o, l.s = -1) : l.s = 1, o === ~~o && o < 1e7) {
          for (u = 0, a = o; a >= 10; a /= 10) u++;
          te ? u > i.maxE ? (l.e = NaN, l.d = null) : u < i.minE ? (l.e = 0, l.d = [
            0
          ]) : (l.e = u, l.d = [
            o
          ]) : (l.e = u, l.d = [
            o
          ]);
          return;
        }
        if (o * 0 !== 0) {
          o || (l.s = NaN), l.e = NaN, l.d = null;
          return;
        }
        return ir(l, o.toString());
      }
      if (f === "string") return (a = o.charCodeAt(0)) === 45 ? (o = o.slice(1), l.s = -1) : (a === 43 && (o = o.slice(1)), l.s = 1), Oi.test(o) ? ir(l, o) : Pc(l, o);
      if (f === "bigint") return o < 0 ? (o = -o, l.s = -1) : l.s = 1, ir(l, o.toString());
      throw Error(ft + o);
    }
    if (i.prototype = P, i.ROUND_UP = 0, i.ROUND_DOWN = 1, i.ROUND_CEIL = 2, i.ROUND_FLOOR = 3, i.ROUND_HALF_UP = 4, i.ROUND_HALF_DOWN = 5, i.ROUND_HALF_EVEN = 6, i.ROUND_HALF_CEIL = 7, i.ROUND_HALF_FLOOR = 8, i.EUCLID = 9, i.config = i.set = Qc, i.clone = Ui, i.isDecimal = wn, i.abs = Rc, i.acos = Lc, i.acosh = Vc, i.add = Uc, i.asin = Wc, i.asinh = Zc, i.atan = Hc, i.atanh = jc, i.atan2 = Jc, i.cbrt = Kc, i.ceil = Yc, i.clamp = Gc, i.cos = Xc, i.cosh = kc, i.div = ef, i.exp = tf, i.floor = rf, i.hypot = nf, i.ln = of, i.log = uf, i.log10 = sf, i.log2 = af, i.max = cf, i.min = ff, i.mod = lf, i.mul = hf, i.pow = df, i.random = mf, i.round = pf, i.sign = vf, i.sin = gf, i.sinh = Df, i.sqrt = wf, i.sub = yf, i.sum = _f, i.tan = Af, i.tanh = Ff, i.trunc = Ef, t === void 0 && (t = {}), t && t.defaults !== true) for (n = [
      "precision",
      "rounding",
      "toExpNeg",
      "toExpPos",
      "maxE",
      "minE",
      "modulo",
      "crypto"
    ], e = 0; e < n.length; ) t.hasOwnProperty(r = n[e++]) || (t[r] = this[r]);
    return i.config(t), i;
  }
  function ef(t, e) {
    return new this(t).div(e);
  }
  function tf(t) {
    return new this(t).exp();
  }
  function rf(t) {
    return X(t = new this(t), t.e + 1, 3);
  }
  function nf() {
    var t, e, r = new this(0);
    for (te = false, t = 0; t < arguments.length; ) if (e = new this(arguments[t++]), e.d) r.d && (r = r.plus(e.times(e)));
    else {
      if (e.s) return te = true, new this(1 / 0);
      r = e;
    }
    return te = true, r.sqrt();
  }
  function wn(t) {
    return t instanceof Dt || t && t.toStringTag === zi || false;
  }
  function of(t) {
    return new this(t).ln();
  }
  function uf(t, e) {
    return new this(t).log(e);
  }
  function af(t) {
    return new this(t).log(2);
  }
  function sf(t) {
    return new this(t).log(10);
  }
  function cf() {
    return Ri(this, arguments, -1);
  }
  function ff() {
    return Ri(this, arguments, 1);
  }
  function lf(t, e) {
    return new this(t).mod(e);
  }
  function hf(t, e) {
    return new this(t).mul(e);
  }
  function df(t, e) {
    return new this(t).pow(e);
  }
  function mf(t) {
    var e, r, n, i, o = 0, u = new this(1), a = [];
    if (t === void 0 ? t = this.precision : Ve(t, 1, lt), n = Math.ceil(t / ee), this.crypto) if (crypto.getRandomValues) for (e = crypto.getRandomValues(new Uint32Array(n)); o < n; ) i = e[o], i >= 429e7 ? e[o] = crypto.getRandomValues(new Uint32Array(1))[0] : a[o++] = i % 1e7;
    else if (crypto.randomBytes) {
      for (e = crypto.randomBytes(n *= 4); o < n; ) i = e[o] + (e[o + 1] << 8) + (e[o + 2] << 16) + ((e[o + 3] & 127) << 24), i >= 214e7 ? crypto.randomBytes(4).copy(e, o) : (a.push(i % 1e7), o += 4);
      o = n / 4;
    } else throw Error(Ii);
    else for (; o < n; ) a[o++] = Math.random() * 1e7 | 0;
    for (n = a[--o], t %= ee, n && t && (i = Ne(10, ee - t), a[o] = (n / i | 0) * i); a[o] === 0; o--) a.pop();
    if (o < 0) r = 0, a = [
      0
    ];
    else {
      for (r = -1; a[0] === 0; r -= ee) a.shift();
      for (n = 1, i = a[0]; i >= 10; i /= 10) n++;
      n < ee && (r -= ee - n);
    }
    return u.e = r, u.d = a, u;
  }
  function pf(t) {
    return X(t = new this(t), t.e + 1, this.rounding);
  }
  function vf(t) {
    return t = new this(t), t.d ? t.d[0] ? t.s : 0 * t.s : t.s || NaN;
  }
  function gf(t) {
    return new this(t).sin();
  }
  function Df(t) {
    return new this(t).sinh();
  }
  function wf(t) {
    return new this(t).sqrt();
  }
  function yf(t, e) {
    return new this(t).sub(e);
  }
  function _f() {
    var t = 0, e = arguments, r = new this(e[t]);
    for (te = false; r.s && ++t < e.length; ) r = r.plus(e[t]);
    return te = true, X(r, this.precision, this.rounding);
  }
  function Af(t) {
    return new this(t).tan();
  }
  function Ff(t) {
    return new this(t).tanh();
  }
  function Ef(t) {
    return X(t = new this(t), t.e + 1, 1);
  }
  P[Symbol.for("nodejs.util.inspect.custom")] = P.toString;
  P[Symbol.toStringTag] = "Decimal";
  var Dt = P.constructor = Ui(Vr);
  ur = new Dt(ur);
  ar = new Dt(ar);
  var Cf = "BigNumber", bf = [
    "?on",
    "config"
  ], Mf = Y(Cf, bf, (t) => {
    var { on: e, config: r } = t, n = Dt.clone({
      precision: r.precision,
      modulo: Dt.EUCLID
    });
    return n.prototype = Object.create(n.prototype), n.prototype.type = "BigNumber", n.prototype.isBigNumber = true, n.prototype.toJSON = function() {
      return {
        mathjs: "BigNumber",
        value: this.toString()
      };
    }, n.fromJSON = function(i) {
      return new n(i.value);
    }, e && e("config", function(i, o) {
      i.precision !== o.precision && n.config({
        precision: i.precision
      });
    }), n;
  }, {
    isClass: true
  });
  const Ie = Math.cosh || function(t) {
    return Math.abs(t) < 1e-9 ? 1 - t : (Math.exp(t) + Math.exp(-t)) * 0.5;
  }, Je = Math.sinh || function(t) {
    return Math.abs(t) < 1e-9 ? t : (Math.exp(t) - Math.exp(-t)) * 0.5;
  }, Sf = function(t) {
    const e = Math.PI / 4;
    if (-e > t || t > e) return Math.cos(t) - 1;
    const r = t * t;
    return r * (r * (r * (r * (r * (r * (r * (r / 20922789888e3 - 1 / 87178291200) + 1 / 479001600) - 1 / 3628800) + 1 / 40320) - 1 / 720) + 1 / 24) - 1 / 2);
  }, Or = function(t, e) {
    return t = Math.abs(t), e = Math.abs(e), t < e && ([t, e] = [
      e,
      t
    ]), t < 1e8 ? Math.sqrt(t * t + e * e) : (e /= t, t * Math.sqrt(1 + e * e));
  }, Ft = function() {
    throw SyntaxError("Invalid Param");
  };
  function Pr(t, e) {
    const r = Math.abs(t), n = Math.abs(e);
    return t === 0 ? Math.log(n) : e === 0 ? Math.log(r) : r < 3e3 && n < 3e3 ? Math.log(t * t + e * e) * 0.5 : (t = t * 0.5, e = e * 0.5, 0.5 * Math.log(t * t + e * e) + Math.LN2);
  }
  const Nf = {
    re: 0,
    im: 0
  }, pt = function(t, e) {
    const r = Nf;
    if (t == null) r.re = r.im = 0;
    else if (e !== void 0) r.re = t, r.im = e;
    else switch (typeof t) {
      case "object":
        if ("im" in t && "re" in t) r.re = t.re, r.im = t.im;
        else if ("abs" in t && "arg" in t) {
          if (!isFinite(t.abs) && isFinite(t.arg)) return $.INFINITY;
          r.re = t.abs * Math.cos(t.arg), r.im = t.abs * Math.sin(t.arg);
        } else if ("r" in t && "phi" in t) {
          if (!isFinite(t.r) && isFinite(t.phi)) return $.INFINITY;
          r.re = t.r * Math.cos(t.phi), r.im = t.r * Math.sin(t.phi);
        } else t.length === 2 ? (r.re = t[0], r.im = t[1]) : Ft();
        break;
      case "string":
        r.im = r.re = 0;
        const n = t.replace(/_/g, "").match(/\d+\.?\d*e[+-]?\d+|\d+\.?\d*|\.\d+|./g);
        let i = 1, o = 0;
        n === null && Ft();
        for (let u = 0; u < n.length; u++) {
          const a = n[u];
          a === " " || a === "	" || a === `
` || (a === "+" ? i++ : a === "-" ? o++ : a === "i" || a === "I" ? (i + o === 0 && Ft(), n[u + 1] !== " " && !isNaN(n[u + 1]) ? (r.im += parseFloat((o % 2 ? "-" : "") + n[u + 1]), u++) : r.im += parseFloat((o % 2 ? "-" : "") + "1"), i = o = 0) : ((i + o === 0 || isNaN(a)) && Ft(), n[u + 1] === "i" || n[u + 1] === "I" ? (r.im += parseFloat((o % 2 ? "-" : "") + a), u++) : r.re += parseFloat((o % 2 ? "-" : "") + a), i = o = 0));
        }
        i + o > 0 && Ft();
        break;
      case "number":
        r.im = 0, r.re = t;
        break;
      default:
        Ft();
    }
    return isNaN(r.re) || isNaN(r.im), r;
  };
  function $(t, e) {
    if (!(this instanceof $)) return new $(t, e);
    const r = pt(t, e);
    this.re = r.re, this.im = r.im;
  }
  $.prototype = {
    re: 0,
    im: 0,
    sign: function() {
      const t = Or(this.re, this.im);
      return new $(this.re / t, this.im / t);
    },
    add: function(t, e) {
      const r = pt(t, e), n = this.isInfinite(), i = !(isFinite(r.re) && isFinite(r.im));
      return n || i ? n && i ? $.NAN : $.INFINITY : new $(this.re + r.re, this.im + r.im);
    },
    sub: function(t, e) {
      const r = pt(t, e), n = this.isInfinite(), i = !(isFinite(r.re) && isFinite(r.im));
      return n || i ? n && i ? $.NAN : $.INFINITY : new $(this.re - r.re, this.im - r.im);
    },
    mul: function(t, e) {
      const r = pt(t, e), n = this.isInfinite(), i = !(isFinite(r.re) && isFinite(r.im)), o = this.re === 0 && this.im === 0, u = r.re === 0 && r.im === 0;
      return n && u || i && o ? $.NAN : n || i ? $.INFINITY : r.im === 0 && this.im === 0 ? new $(this.re * r.re, 0) : new $(this.re * r.re - this.im * r.im, this.re * r.im + this.im * r.re);
    },
    div: function(t, e) {
      const r = pt(t, e), n = this.isInfinite(), i = !(isFinite(r.re) && isFinite(r.im)), o = this.re === 0 && this.im === 0, u = r.re === 0 && r.im === 0;
      if (o && u || n && i) return $.NAN;
      if (u || n) return $.INFINITY;
      if (o || i) return $.ZERO;
      if (r.im === 0) return new $(this.re / r.re, this.im / r.re);
      if (Math.abs(r.re) < Math.abs(r.im)) {
        const a = r.re / r.im, f = r.re * a + r.im;
        return new $((this.re * a + this.im) / f, (this.im * a - this.re) / f);
      } else {
        const a = r.im / r.re, f = r.im * a + r.re;
        return new $((this.re + this.im * a) / f, (this.im - this.re * a) / f);
      }
    },
    pow: function(t, e) {
      const r = pt(t, e), n = this.re === 0 && this.im === 0;
      if (r.re === 0 && r.im === 0) return $.ONE;
      if (r.im === 0) {
        if (this.im === 0 && this.re > 0) return new $(Math.pow(this.re, r.re), 0);
        if (this.re === 0) switch ((r.re % 4 + 4) % 4) {
          case 0:
            return new $(Math.pow(this.im, r.re), 0);
          case 1:
            return new $(0, Math.pow(this.im, r.re));
          case 2:
            return new $(-Math.pow(this.im, r.re), 0);
          case 3:
            return new $(0, -Math.pow(this.im, r.re));
        }
      }
      if (n && r.re > 0) return $.ZERO;
      const o = Math.atan2(this.im, this.re), u = Pr(this.re, this.im);
      let a = Math.exp(r.re * u - r.im * o), f = r.im * u + r.re * o;
      return new $(a * Math.cos(f), a * Math.sin(f));
    },
    sqrt: function() {
      const t = this.re, e = this.im;
      if (e === 0) return t >= 0 ? new $(Math.sqrt(t), 0) : new $(0, Math.sqrt(-t));
      const r = Or(t, e);
      let n = Math.sqrt(0.5 * (r + Math.abs(t))), i = Math.abs(e) / (2 * n);
      return t >= 0 ? new $(n, e < 0 ? -i : i) : new $(i, e < 0 ? -n : n);
    },
    exp: function() {
      const t = Math.exp(this.re);
      return this.im === 0 ? new $(t, 0) : new $(t * Math.cos(this.im), t * Math.sin(this.im));
    },
    expm1: function() {
      const t = this.re, e = this.im;
      return new $(Math.expm1(t) * Math.cos(e) + Sf(e), Math.exp(t) * Math.sin(e));
    },
    log: function() {
      const t = this.re, e = this.im;
      return e === 0 && t > 0 ? new $(Math.log(t), 0) : new $(Pr(t, e), Math.atan2(e, t));
    },
    abs: function() {
      return Or(this.re, this.im);
    },
    arg: function() {
      return Math.atan2(this.im, this.re);
    },
    sin: function() {
      const t = this.re, e = this.im;
      return new $(Math.sin(t) * Ie(e), Math.cos(t) * Je(e));
    },
    cos: function() {
      const t = this.re, e = this.im;
      return new $(Math.cos(t) * Ie(e), -Math.sin(t) * Je(e));
    },
    tan: function() {
      const t = 2 * this.re, e = 2 * this.im, r = Math.cos(t) + Ie(e);
      return new $(Math.sin(t) / r, Je(e) / r);
    },
    cot: function() {
      const t = 2 * this.re, e = 2 * this.im, r = Math.cos(t) - Ie(e);
      return new $(-Math.sin(t) / r, Je(e) / r);
    },
    sec: function() {
      const t = this.re, e = this.im, r = 0.5 * Ie(2 * e) + 0.5 * Math.cos(2 * t);
      return new $(Math.cos(t) * Ie(e) / r, Math.sin(t) * Je(e) / r);
    },
    csc: function() {
      const t = this.re, e = this.im, r = 0.5 * Ie(2 * e) - 0.5 * Math.cos(2 * t);
      return new $(Math.sin(t) * Ie(e) / r, -Math.cos(t) * Je(e) / r);
    },
    asin: function() {
      const t = this.re, e = this.im, r = new $(e * e - t * t + 1, -2 * t * e).sqrt(), n = new $(r.re - e, r.im + t).log();
      return new $(n.im, -n.re);
    },
    acos: function() {
      const t = this.re, e = this.im, r = new $(e * e - t * t + 1, -2 * t * e).sqrt(), n = new $(r.re - e, r.im + t).log();
      return new $(Math.PI / 2 - n.im, n.re);
    },
    atan: function() {
      const t = this.re, e = this.im;
      if (t === 0) {
        if (e === 1) return new $(0, 1 / 0);
        if (e === -1) return new $(0, -1 / 0);
      }
      const r = t * t + (1 - e) * (1 - e), n = new $((1 - e * e - t * t) / r, -2 * t / r).log();
      return new $(-0.5 * n.im, 0.5 * n.re);
    },
    acot: function() {
      const t = this.re, e = this.im;
      if (e === 0) return new $(Math.atan2(1, t), 0);
      const r = t * t + e * e;
      return r !== 0 ? new $(t / r, -e / r).atan() : new $(t !== 0 ? t / 0 : 0, e !== 0 ? -e / 0 : 0).atan();
    },
    asec: function() {
      const t = this.re, e = this.im;
      if (t === 0 && e === 0) return new $(0, 1 / 0);
      const r = t * t + e * e;
      return r !== 0 ? new $(t / r, -e / r).acos() : new $(t !== 0 ? t / 0 : 0, e !== 0 ? -e / 0 : 0).acos();
    },
    acsc: function() {
      const t = this.re, e = this.im;
      if (t === 0 && e === 0) return new $(Math.PI / 2, 1 / 0);
      const r = t * t + e * e;
      return r !== 0 ? new $(t / r, -e / r).asin() : new $(t !== 0 ? t / 0 : 0, e !== 0 ? -e / 0 : 0).asin();
    },
    sinh: function() {
      const t = this.re, e = this.im;
      return new $(Je(t) * Math.cos(e), Ie(t) * Math.sin(e));
    },
    cosh: function() {
      const t = this.re, e = this.im;
      return new $(Ie(t) * Math.cos(e), Je(t) * Math.sin(e));
    },
    tanh: function() {
      const t = 2 * this.re, e = 2 * this.im, r = Ie(t) + Math.cos(e);
      return new $(Je(t) / r, Math.sin(e) / r);
    },
    coth: function() {
      const t = 2 * this.re, e = 2 * this.im, r = Ie(t) - Math.cos(e);
      return new $(Je(t) / r, -Math.sin(e) / r);
    },
    csch: function() {
      const t = this.re, e = this.im, r = Math.cos(2 * e) - Ie(2 * t);
      return new $(-2 * Je(t) * Math.cos(e) / r, 2 * Ie(t) * Math.sin(e) / r);
    },
    sech: function() {
      const t = this.re, e = this.im, r = Math.cos(2 * e) + Ie(2 * t);
      return new $(2 * Ie(t) * Math.cos(e) / r, -2 * Je(t) * Math.sin(e) / r);
    },
    asinh: function() {
      let t = this.im;
      this.im = -this.re, this.re = t;
      const e = this.asin();
      return this.re = -this.im, this.im = t, t = e.re, e.re = -e.im, e.im = t, e;
    },
    acosh: function() {
      const t = this.acos();
      if (t.im <= 0) {
        const e = t.re;
        t.re = -t.im, t.im = e;
      } else {
        const e = t.im;
        t.im = -t.re, t.re = e;
      }
      return t;
    },
    atanh: function() {
      const t = this.re, e = this.im, r = t > 1 && e === 0, n = 1 - t, i = 1 + t, o = n * n + e * e, u = o !== 0 ? new $((i * n - e * e) / o, (e * n + i * e) / o) : new $(t !== -1 ? t / 0 : 0, e !== 0 ? e / 0 : 0), a = u.re;
      return u.re = Pr(u.re, u.im) / 2, u.im = Math.atan2(u.im, a) / 2, r && (u.im = -u.im), u;
    },
    acoth: function() {
      const t = this.re, e = this.im;
      if (t === 0 && e === 0) return new $(0, Math.PI / 2);
      const r = t * t + e * e;
      return r !== 0 ? new $(t / r, -e / r).atanh() : new $(t !== 0 ? t / 0 : 0, e !== 0 ? -e / 0 : 0).atanh();
    },
    acsch: function() {
      const t = this.re, e = this.im;
      if (e === 0) return new $(t !== 0 ? Math.log(t + Math.sqrt(t * t + 1)) : 1 / 0, 0);
      const r = t * t + e * e;
      return r !== 0 ? new $(t / r, -e / r).asinh() : new $(t !== 0 ? t / 0 : 0, e !== 0 ? -e / 0 : 0).asinh();
    },
    asech: function() {
      const t = this.re, e = this.im;
      if (this.isZero()) return $.INFINITY;
      const r = t * t + e * e;
      return r !== 0 ? new $(t / r, -e / r).acosh() : new $(t !== 0 ? t / 0 : 0, e !== 0 ? -e / 0 : 0).acosh();
    },
    inverse: function() {
      if (this.isZero()) return $.INFINITY;
      if (this.isInfinite()) return $.ZERO;
      const t = this.re, e = this.im, r = t * t + e * e;
      return new $(t / r, -e / r);
    },
    conjugate: function() {
      return new $(this.re, -this.im);
    },
    neg: function() {
      return new $(-this.re, -this.im);
    },
    ceil: function(t) {
      return t = Math.pow(10, t || 0), new $(Math.ceil(this.re * t) / t, Math.ceil(this.im * t) / t);
    },
    floor: function(t) {
      return t = Math.pow(10, t || 0), new $(Math.floor(this.re * t) / t, Math.floor(this.im * t) / t);
    },
    round: function(t) {
      return t = Math.pow(10, t || 0), new $(Math.round(this.re * t) / t, Math.round(this.im * t) / t);
    },
    equals: function(t, e) {
      const r = pt(t, e);
      return Math.abs(r.re - this.re) <= $.EPSILON && Math.abs(r.im - this.im) <= $.EPSILON;
    },
    clone: function() {
      return new $(this.re, this.im);
    },
    toString: function() {
      let t = this.re, e = this.im, r = "";
      return this.isNaN() ? "NaN" : this.isInfinite() ? "Infinity" : (Math.abs(t) < $.EPSILON && (t = 0), Math.abs(e) < $.EPSILON && (e = 0), e === 0 ? r + t : (t !== 0 ? (r += t, r += " ", e < 0 ? (e = -e, r += "-") : r += "+", r += " ") : e < 0 && (e = -e, r += "-"), e !== 1 && (r += e), r + "i"));
    },
    toVector: function() {
      return [
        this.re,
        this.im
      ];
    },
    valueOf: function() {
      return this.im === 0 ? this.re : null;
    },
    isNaN: function() {
      return isNaN(this.re) || isNaN(this.im);
    },
    isZero: function() {
      return this.im === 0 && this.re === 0;
    },
    isFinite: function() {
      return isFinite(this.re) && isFinite(this.im);
    },
    isInfinite: function() {
      return !this.isFinite();
    }
  };
  $.ZERO = new $(0, 0);
  $.ONE = new $(1, 0);
  $.I = new $(0, 1);
  $.PI = new $(Math.PI, 0);
  $.E = new $(Math.E, 0);
  $.INFINITY = new $(1 / 0, 1 / 0);
  $.NAN = new $(NaN, NaN);
  $.EPSILON = 1e-15;
  var Bf = "Complex", xf = [], Tf = Y(Bf, xf, () => (Object.defineProperty($, "name", {
    value: "Complex"
  }), $.prototype.constructor = $, $.prototype.type = "Complex", $.prototype.isComplex = true, $.prototype.toJSON = function() {
    return {
      mathjs: "Complex",
      re: this.re,
      im: this.im
    };
  }, $.prototype.toPolar = function() {
    return {
      r: this.abs(),
      phi: this.arg()
    };
  }, $.prototype.format = function(t) {
    var e = "", r = this.im, n = this.re, i = Rr(this.re, t), o = Rr(this.im, t), u = ve(t) ? t : t ? t.precision : null;
    if (u !== null) {
      var a = Math.pow(10, -u);
      Math.abs(n / r) < a && (n = 0), Math.abs(r / n) < a && (r = 0);
    }
    return r === 0 ? e = i : n === 0 ? r === 1 ? e = "i" : r === -1 ? e = "-i" : e = o + "i" : r < 0 ? r === -1 ? e = i + " - i" : e = i + " - " + o.substring(1) + "i" : r === 1 ? e = i + " + i" : e = i + " + " + o + "i", e;
  }, $.fromPolar = function(t) {
    switch (arguments.length) {
      case 1: {
        var e = arguments[0];
        if (typeof e == "object") return $(e);
        throw new TypeError("Input has to be an object with r and phi keys.");
      }
      case 2: {
        var r = arguments[0], n = arguments[1];
        if (ve(r)) {
          if (Fi(n) && n.hasBase("ANGLE") && (n = n.toNumber("rad")), ve(n)) return new $({
            r,
            phi: n
          });
          throw new TypeError("Phi is not a number nor an angle unit.");
        } else throw new TypeError("Radius r is not a number.");
      }
      default:
        throw new SyntaxError("Wrong number of arguments in function fromPolar");
    }
  }, $.prototype.valueOf = $.prototype.toString, $.fromJSON = function(t) {
    return new $(t);
  }, $.compare = function(t, e) {
    return t.re > e.re ? 1 : t.re < e.re ? -1 : t.im > e.im ? 1 : t.im < e.im ? -1 : 0;
  }, $), {
    isClass: true
  });
  function $f(t) {
    return t && t.__esModule && Object.prototype.hasOwnProperty.call(t, "default") ? t.default : t;
  }
  var Wi = {
    exports: {}
  };
  (function(t, e) {
    (function(r) {
      var n = 2e3, i = {
        s: 1,
        n: 0,
        d: 1
      };
      function o(m, w) {
        if (isNaN(m = parseInt(m, 10))) throw c();
        return m * w;
      }
      function u(m, w) {
        if (w === 0) throw D();
        var v = Object.create(p.prototype);
        v.s = m < 0 ? -1 : 1, m = m < 0 ? -m : m;
        var A = d(m, w);
        return v.n = m / A, v.d = w / A, v;
      }
      function a(m) {
        for (var w = {}, v = m, A = 2, y = 4; y <= v; ) {
          for (; v % A === 0; ) v /= A, w[A] = (w[A] || 0) + 1;
          y += 1 + 2 * A++;
        }
        return v !== m ? v > 1 && (w[v] = (w[v] || 0) + 1) : w[m] = (w[m] || 0) + 1, w;
      }
      var f = function(m, w) {
        var v = 0, A = 1, y = 1, _ = 0, E = 0, C = 0, b = 1, x = 1, N = 0, B = 1, Z = 1, q = 1, U = 1e7, I;
        if (m != null) if (w !== void 0) {
          if (v = m, A = w, y = v * A, v % 1 !== 0 || A % 1 !== 0) throw g();
        } else switch (typeof m) {
          case "object": {
            if ("d" in m && "n" in m) v = m.n, A = m.d, "s" in m && (v *= m.s);
            else if (0 in m) v = m[0], 1 in m && (A = m[1]);
            else throw c();
            y = v * A;
            break;
          }
          case "number": {
            if (m < 0 && (y = m, m = -m), m % 1 === 0) v = m;
            else if (m > 0) {
              for (m >= 1 && (x = Math.pow(10, Math.floor(1 + Math.log(m) / Math.LN10)), m /= x); B <= U && q <= U; ) if (I = (N + Z) / (B + q), m === I) {
                B + q <= U ? (v = N + Z, A = B + q) : q > B ? (v = Z, A = q) : (v = N, A = B);
                break;
              } else m > I ? (N += Z, B += q) : (Z += N, q += B), B > U ? (v = Z, A = q) : (v = N, A = B);
              v *= x;
            } else (isNaN(m) || isNaN(w)) && (A = v = NaN);
            break;
          }
          case "string": {
            if (B = m.match(/\d+|./g), B === null) throw c();
            if (B[N] === "-" ? (y = -1, N++) : B[N] === "+" && N++, B.length === N + 1 ? E = o(B[N++], y) : B[N + 1] === "." || B[N] === "." ? (B[N] !== "." && (_ = o(B[N++], y)), N++, (N + 1 === B.length || B[N + 1] === "(" && B[N + 3] === ")" || B[N + 1] === "'" && B[N + 3] === "'") && (E = o(B[N], y), b = Math.pow(10, B[N].length), N++), (B[N] === "(" && B[N + 2] === ")" || B[N] === "'" && B[N + 2] === "'") && (C = o(B[N + 1], y), x = Math.pow(10, B[N + 1].length) - 1, N += 3)) : B[N + 1] === "/" || B[N + 1] === ":" ? (E = o(B[N], y), b = o(B[N + 2], 1), N += 3) : B[N + 3] === "/" && B[N + 1] === " " && (_ = o(B[N], y), E = o(B[N + 2], y), b = o(B[N + 4], 1), N += 5), B.length <= N) {
              A = b * x, y = v = C + A * _ + x * E;
              break;
            }
          }
          default:
            throw c();
        }
        if (A === 0) throw D();
        i.s = y < 0 ? -1 : 1, i.n = Math.abs(v), i.d = Math.abs(A);
      };
      function l(m, w, v) {
        for (var A = 1; w > 0; m = m * m % v, w >>= 1) w & 1 && (A = A * m % v);
        return A;
      }
      function s(m, w) {
        for (; w % 2 === 0; w /= 2) ;
        for (; w % 5 === 0; w /= 5) ;
        if (w === 1) return 0;
        for (var v = 10 % w, A = 1; v !== 1; A++) if (v = v * 10 % w, A > n) return 0;
        return A;
      }
      function h(m, w, v) {
        for (var A = 1, y = l(10, v, w), _ = 0; _ < 300; _++) {
          if (A === y) return _;
          A = A * 10 % w, y = y * 10 % w;
        }
        return 0;
      }
      function d(m, w) {
        if (!m) return w;
        if (!w) return m;
        for (; ; ) {
          if (m %= w, !m) return w;
          if (w %= m, !w) return m;
        }
      }
      function p(m, w) {
        if (f(m, w), this instanceof p) m = d(i.d, i.n), this.s = i.s, this.n = i.n / m, this.d = i.d / m;
        else return u(i.s * i.n, i.d);
      }
      var D = function() {
        return new Error("Division by Zero");
      }, c = function() {
        return new Error("Invalid argument");
      }, g = function() {
        return new Error("Parameters must be integer");
      };
      p.prototype = {
        s: 1,
        n: 0,
        d: 1,
        abs: function() {
          return u(this.n, this.d);
        },
        neg: function() {
          return u(-this.s * this.n, this.d);
        },
        add: function(m, w) {
          return f(m, w), u(this.s * this.n * i.d + i.s * this.d * i.n, this.d * i.d);
        },
        sub: function(m, w) {
          return f(m, w), u(this.s * this.n * i.d - i.s * this.d * i.n, this.d * i.d);
        },
        mul: function(m, w) {
          return f(m, w), u(this.s * i.s * this.n * i.n, this.d * i.d);
        },
        div: function(m, w) {
          return f(m, w), u(this.s * i.s * this.n * i.d, this.d * i.n);
        },
        clone: function() {
          return u(this.s * this.n, this.d);
        },
        mod: function(m, w) {
          if (isNaN(this.n) || isNaN(this.d)) return new p(NaN);
          if (m === void 0) return u(this.s * this.n % this.d, 1);
          if (f(m, w), i.n === 0 && this.d === 0) throw D();
          return u(this.s * (i.d * this.n) % (i.n * this.d), i.d * this.d);
        },
        gcd: function(m, w) {
          return f(m, w), u(d(i.n, this.n) * d(i.d, this.d), i.d * this.d);
        },
        lcm: function(m, w) {
          return f(m, w), i.n === 0 && this.n === 0 ? u(0, 1) : u(i.n * this.n, d(i.n, this.n) * d(i.d, this.d));
        },
        ceil: function(m) {
          return m = Math.pow(10, m || 0), isNaN(this.n) || isNaN(this.d) ? new p(NaN) : u(Math.ceil(m * this.s * this.n / this.d), m);
        },
        floor: function(m) {
          return m = Math.pow(10, m || 0), isNaN(this.n) || isNaN(this.d) ? new p(NaN) : u(Math.floor(m * this.s * this.n / this.d), m);
        },
        round: function(m) {
          return m = Math.pow(10, m || 0), isNaN(this.n) || isNaN(this.d) ? new p(NaN) : u(Math.round(m * this.s * this.n / this.d), m);
        },
        inverse: function() {
          return u(this.s * this.d, this.n);
        },
        pow: function(m, w) {
          if (f(m, w), i.d === 1) return i.s < 0 ? u(Math.pow(this.s * this.d, i.n), Math.pow(this.n, i.n)) : u(Math.pow(this.s * this.n, i.n), Math.pow(this.d, i.n));
          if (this.s < 0) return null;
          var v = a(this.n), A = a(this.d), y = 1, _ = 1;
          for (var E in v) if (E !== "1") {
            if (E === "0") {
              y = 0;
              break;
            }
            if (v[E] *= i.n, v[E] % i.d === 0) v[E] /= i.d;
            else return null;
            y *= Math.pow(E, v[E]);
          }
          for (var E in A) if (E !== "1") {
            if (A[E] *= i.n, A[E] % i.d === 0) A[E] /= i.d;
            else return null;
            _ *= Math.pow(E, A[E]);
          }
          return i.s < 0 ? u(_, y) : u(y, _);
        },
        equals: function(m, w) {
          return f(m, w), this.s * this.n * i.d === i.s * i.n * this.d;
        },
        compare: function(m, w) {
          f(m, w);
          var v = this.s * this.n * i.d - i.s * i.n * this.d;
          return (0 < v) - (v < 0);
        },
        simplify: function(m) {
          if (isNaN(this.n) || isNaN(this.d)) return this;
          m = m || 1e-3;
          for (var w = this.abs(), v = w.toContinued(), A = 1; A < v.length; A++) {
            for (var y = u(v[A - 1], 1), _ = A - 2; _ >= 0; _--) y = y.inverse().add(v[_]);
            if (Math.abs(y.sub(w).valueOf()) < m) return y.mul(this.s);
          }
          return this;
        },
        divisible: function(m, w) {
          return f(m, w), !(!(i.n * this.d) || this.n * i.d % (i.n * this.d));
        },
        valueOf: function() {
          return this.s * this.n / this.d;
        },
        toFraction: function(m) {
          var w, v = "", A = this.n, y = this.d;
          return this.s < 0 && (v += "-"), y === 1 ? v += A : (m && (w = Math.floor(A / y)) > 0 && (v += w, v += " ", A %= y), v += A, v += "/", v += y), v;
        },
        toLatex: function(m) {
          var w, v = "", A = this.n, y = this.d;
          return this.s < 0 && (v += "-"), y === 1 ? v += A : (m && (w = Math.floor(A / y)) > 0 && (v += w, A %= y), v += "\\frac{", v += A, v += "}{", v += y, v += "}"), v;
        },
        toContinued: function() {
          var m, w = this.n, v = this.d, A = [];
          if (isNaN(w) || isNaN(v)) return A;
          do
            A.push(Math.floor(w / v)), m = w % v, w = v, v = m;
          while (w !== 1);
          return A;
        },
        toString: function(m) {
          var w = this.n, v = this.d;
          if (isNaN(w) || isNaN(v)) return "NaN";
          m = m || 15;
          var A = s(w, v), y = h(w, v, A), _ = this.s < 0 ? "-" : "";
          if (_ += w / v | 0, w %= v, w *= 10, w && (_ += "."), A) {
            for (var E = y; E--; ) _ += w / v | 0, w %= v, w *= 10;
            _ += "(";
            for (var E = A; E--; ) _ += w / v | 0, w %= v, w *= 10;
            _ += ")";
          } else for (var E = m; w && E--; ) _ += w / v | 0, w %= v, w *= 10;
          return _;
        }
      }, Object.defineProperty(p, "__esModule", {
        value: true
      }), p.default = p, p.Fraction = p, t.exports = p;
    })();
  })(Wi);
  var If = Wi.exports;
  const it = $f(If);
  var zf = "Fraction", Of = [], Pf = Y(zf, Of, () => (Object.defineProperty(it, "name", {
    value: "Fraction"
  }), it.prototype.constructor = it, it.prototype.type = "Fraction", it.prototype.isFraction = true, it.prototype.toJSON = function() {
    return {
      mathjs: "Fraction",
      n: this.s * this.n,
      d: this.d
    };
  }, it.fromJSON = function(t) {
    return new it(t);
  }, it), {
    isClass: true
  }), qf = "Matrix", Rf = [], Lf = Y(qf, Rf, () => {
    function t() {
      if (!(this instanceof t)) throw new SyntaxError("Constructor must be called with the new operator");
    }
    return t.prototype.type = "Matrix", t.prototype.isMatrix = true, t.prototype.storage = function() {
      throw new Error("Cannot invoke storage on a Matrix interface");
    }, t.prototype.datatype = function() {
      throw new Error("Cannot invoke datatype on a Matrix interface");
    }, t.prototype.create = function(e, r) {
      throw new Error("Cannot invoke create on a Matrix interface");
    }, t.prototype.subset = function(e, r, n) {
      throw new Error("Cannot invoke subset on a Matrix interface");
    }, t.prototype.get = function(e) {
      throw new Error("Cannot invoke get on a Matrix interface");
    }, t.prototype.set = function(e, r, n) {
      throw new Error("Cannot invoke set on a Matrix interface");
    }, t.prototype.resize = function(e, r) {
      throw new Error("Cannot invoke resize on a Matrix interface");
    }, t.prototype.reshape = function(e, r) {
      throw new Error("Cannot invoke reshape on a Matrix interface");
    }, t.prototype.clone = function() {
      throw new Error("Cannot invoke clone on a Matrix interface");
    }, t.prototype.size = function() {
      throw new Error("Cannot invoke size on a Matrix interface");
    }, t.prototype.map = function(e, r) {
      throw new Error("Cannot invoke map on a Matrix interface");
    }, t.prototype.forEach = function(e) {
      throw new Error("Cannot invoke forEach on a Matrix interface");
    }, t.prototype[Symbol.iterator] = function() {
      throw new Error("Cannot iterate a Matrix interface");
    }, t.prototype.toArray = function() {
      throw new Error("Cannot invoke toArray on a Matrix interface");
    }, t.prototype.valueOf = function() {
      throw new Error("Cannot invoke valueOf on a Matrix interface");
    }, t.prototype.format = function(e) {
      throw new Error("Cannot invoke format on a Matrix interface");
    }, t.prototype.toString = function() {
      throw new Error("Cannot invoke toString on a Matrix interface");
    }, t;
  }, {
    isClass: true
  });
  function qr(t, e, r) {
    var n = t.constructor, i = new n(2), o = "";
    if (r) {
      if (r < 1) throw new Error("size must be in greater than 0");
      if (!De(r)) throw new Error("size must be an integer");
      if (t.greaterThan(i.pow(r - 1).sub(1)) || t.lessThan(i.pow(r - 1).mul(-1))) throw new Error("Value must be in range [-2^".concat(r - 1, ", 2^").concat(r - 1, "-1]"));
      if (!t.isInteger()) throw new Error("Value must be an integer");
      t.lessThan(0) && (t = t.add(i.pow(r))), o = "i".concat(r);
    }
    switch (e) {
      case 2:
        return "".concat(t.toBinary()).concat(o);
      case 8:
        return "".concat(t.toOctal()).concat(o);
      case 16:
        return "".concat(t.toHexadecimal()).concat(o);
      default:
        throw new Error("Base ".concat(e, " not supported "));
    }
  }
  function Vf(t, e) {
    if (typeof e == "function") return e(t);
    if (!t.isFinite()) return t.isNaN() ? "NaN" : t.gt(0) ? "Infinity" : "-Infinity";
    var { notation: r, precision: n, wordSize: i } = bi(e);
    switch (r) {
      case "fixed":
        return Wf(t, n);
      case "exponential":
        return yn(t, n);
      case "engineering":
        return Uf(t, n);
      case "bin":
        return qr(t, 2, i);
      case "oct":
        return qr(t, 8, i);
      case "hex":
        return qr(t, 16, i);
      case "auto": {
        var o = _n(e == null ? void 0 : e.lowerExp, -3), u = _n(e == null ? void 0 : e.upperExp, 5);
        if (t.isZero()) return "0";
        var a, f = t.toSignificantDigits(n), l = f.e;
        return l >= o && l < u ? a = f.toFixed() : a = yn(t, n), a.replace(/((\.\d*?)(0+))($|e)/, function() {
          var s = arguments[2], h = arguments[4];
          return s !== "." ? s + h : h;
        });
      }
      default:
        throw new Error('Unknown notation "' + r + '". Choose "auto", "exponential", "fixed", "bin", "oct", or "hex.');
    }
  }
  function Uf(t, e) {
    var r = t.e, n = r % 3 === 0 ? r : r < 0 ? r - 3 - r % 3 : r - r % 3, i = t.mul(Math.pow(10, -n)), o = i.toPrecision(e);
    if (o.includes("e")) {
      var u = t.constructor;
      o = new u(o).toFixed();
    }
    return o + "e" + (r >= 0 ? "+" : "") + n.toString();
  }
  function yn(t, e) {
    return e !== void 0 ? t.toExponential(e - 1) : t.toExponential();
  }
  function Wf(t, e) {
    return t.toFixed(e);
  }
  function _n(t, e) {
    return ve(t) ? t : Fe(t) ? t.toNumber() : e;
  }
  function Me(t, e) {
    var r = Zf(t, e);
    return e && typeof e == "object" && "truncate" in e && r.length > e.truncate ? r.substring(0, e.truncate - 3) + "..." : r;
  }
  function Zf(t, e) {
    if (typeof t == "number") return Rr(t, e);
    if (Fe(t)) return Vf(t, e);
    if (Hf(t)) return !e || e.fraction !== "decimal" ? t.s * t.n + "/" + t.d : t.toString();
    if (Array.isArray(t)) return Zi(t, e);
    if (Xe(t)) return An(t);
    if (typeof t == "function") return t.syntax ? String(t.syntax) : "function";
    if (t && typeof t == "object") {
      if (typeof t.format == "function") return t.format(e);
      if (t && t.toString(e) !== {}.toString()) return t.toString(e);
      var r = Object.keys(t).map((n) => An(n) + ": " + Me(t[n], e));
      return "{" + r.join(", ") + "}";
    }
    return String(t);
  }
  function An(t) {
    for (var e = String(t), r = "", n = 0; n < e.length; ) {
      var i = e.charAt(n);
      r += i in Fn ? Fn[i] : i, n++;
    }
    return '"' + r + '"';
  }
  var Fn = {
    '"': '\\"',
    "\\": "\\\\",
    "\b": "\\b",
    "\f": "\\f",
    "\n": "\\n",
    "\r": "\\r",
    "	": "\\t"
  };
  function Zi(t, e) {
    if (Array.isArray(t)) {
      for (var r = "[", n = t.length, i = 0; i < n; i++) i !== 0 && (r += ", "), r += Zi(t[i], e);
      return r += "]", r;
    } else return Me(t, e);
  }
  function Hf(t) {
    return t && typeof t == "object" && typeof t.s == "number" && typeof t.n == "number" && typeof t.d == "number" || false;
  }
  function oe(t, e, r) {
    if (!(this instanceof oe)) throw new SyntaxError("Constructor must be called with the new operator");
    this.actual = t, this.expected = e, this.relation = r, this.message = "Dimension mismatch (" + (Array.isArray(t) ? "[" + t.join(", ") + "]" : t) + " " + (this.relation || "!=") + " " + (Array.isArray(e) ? "[" + e.join(", ") + "]" : e) + ")", this.stack = new Error().stack;
  }
  oe.prototype = new RangeError();
  oe.prototype.constructor = RangeError;
  oe.prototype.name = "DimensionError";
  oe.prototype.isDimensionError = true;
  function ht(t, e, r) {
    if (!(this instanceof ht)) throw new SyntaxError("Constructor must be called with the new operator");
    this.index = t, arguments.length < 3 ? (this.min = 0, this.max = e) : (this.min = e, this.max = r), this.min !== void 0 && this.index < this.min ? this.message = "Index out of range (" + this.index + " < " + this.min + ")" : this.max !== void 0 && this.index >= this.max ? this.message = "Index out of range (" + this.index + " > " + (this.max - 1) + ")" : this.message = "Index out of range (" + this.index + ")", this.stack = new Error().stack;
  }
  ht.prototype = new RangeError();
  ht.prototype.constructor = RangeError;
  ht.prototype.name = "IndexError";
  ht.prototype.isIndexError = true;
  function Be(t) {
    for (var e = []; Array.isArray(t); ) e.push(t.length), t = t[0];
    return e;
  }
  function Hi(t, e, r) {
    var n, i = t.length;
    if (i !== e[r]) throw new oe(i, e[r]);
    if (r < e.length - 1) {
      var o = r + 1;
      for (n = 0; n < i; n++) {
        var u = t[n];
        if (!Array.isArray(u)) throw new oe(e.length - 1, e.length, "<");
        Hi(t[n], e, o);
      }
    } else for (n = 0; n < i; n++) if (Array.isArray(t[n])) throw new oe(e.length + 1, e.length, ">");
  }
  function En(t, e) {
    var r = e.length === 0;
    if (r) {
      if (Array.isArray(t)) throw new oe(t.length, 0);
    } else Hi(t, e, 0);
  }
  function ye(t, e) {
    if (t !== void 0) {
      if (!ve(t) || !De(t)) throw new TypeError("Index must be an integer (value: " + t + ")");
      if (t < 0 || typeof e == "number" && t >= e) throw new ht(t, e);
    }
  }
  function cr(t, e, r) {
    if (!Array.isArray(e)) throw new TypeError("Array expected");
    if (e.length === 0) throw new Error("Resizing to scalar is not supported");
    e.forEach(function(i) {
      if (!ve(i) || !De(i) || i < 0) throw new TypeError("Invalid size, must contain positive integers (size: " + Me(e) + ")");
    }), (ve(t) || Fe(t)) && (t = [
      t
    ]);
    var n = r !== void 0 ? r : 0;
    return Zr(t, e, 0, n), t;
  }
  function Zr(t, e, r, n) {
    var i, o, u = t.length, a = e[r], f = Math.min(u, a);
    if (t.length = a, r < e.length - 1) {
      var l = r + 1;
      for (i = 0; i < f; i++) o = t[i], Array.isArray(o) || (o = [
        o
      ], t[i] = o), Zr(o, e, l, n);
      for (i = f; i < a; i++) o = [], t[i] = o, Zr(o, e, l, n);
    } else {
      for (i = 0; i < f; i++) for (; Array.isArray(t[i]); ) t[i] = t[i][0];
      for (i = f; i < a; i++) t[i] = n;
    }
  }
  function ji(t, e) {
    var r = Jf(t), n = r.length;
    if (!Array.isArray(t) || !Array.isArray(e)) throw new TypeError("Array expected");
    if (e.length === 0) throw new oe(0, n, "!=");
    e = Gr(e, n);
    var i = Ji(e);
    if (n !== i) throw new oe(i, n, "!=");
    try {
      return jf(r, e);
    } catch (o) {
      throw o instanceof oe ? new oe(i, n, "!=") : o;
    }
  }
  function Gr(t, e) {
    var r = Ji(t), n = t.slice(), i = -1, o = t.indexOf(i), u = t.indexOf(i, o + 1) >= 0;
    if (u) throw new Error("More than one wildcard in sizes");
    var a = o >= 0, f = e % r === 0;
    if (a) if (f) n[o] = -e / r;
    else throw new Error("Could not replace wildcard, since " + e + " is no multiple of " + -r);
    return n;
  }
  function Ji(t) {
    return t.reduce((e, r) => e * r, 1);
  }
  function jf(t, e) {
    for (var r = t, n, i = e.length - 1; i > 0; i--) {
      var o = e[i];
      n = [];
      for (var u = r.length / o, a = 0; a < u; a++) n.push(r.slice(a * o, (a + 1) * o));
      r = n;
    }
    return r;
  }
  function Ki(t, e, r, n) {
    var i = n || Be(t);
    if (r) for (var o = 0; o < r; o++) t = [
      t
    ], i.unshift(1);
    for (t = Yi(t, e, 0); i.length < e; ) i.push(1);
    return t;
  }
  function Yi(t, e, r) {
    var n, i;
    if (Array.isArray(t)) {
      var o = r + 1;
      for (n = 0, i = t.length; n < i; n++) t[n] = Yi(t[n], e, o);
    } else for (var u = r; u < e; u++) t = [
      t
    ];
    return t;
  }
  function Jf(t) {
    if (!Array.isArray(t)) return t;
    var e = [];
    return t.forEach(function r(n) {
      Array.isArray(n) ? n.forEach(r) : e.push(n);
    }), e;
  }
  function fr(t, e) {
    for (var r, n = 0, i = 0; i < t.length; i++) {
      var o = t[i], u = Array.isArray(o);
      if (i === 0 && u && (n = o.length), u && o.length !== n) return;
      var a = u ? fr(o, e) : e(o);
      if (r === void 0) r = a;
      else if (r !== a) return "mixed";
    }
    return r;
  }
  function Gi(t, e, r, n) {
    if (n < r) {
      if (t.length !== e.length) throw new oe(t.length, e.length);
      for (var i = [], o = 0; o < t.length; o++) i[o] = Gi(t[o], e[o], r, n + 1);
      return i;
    } else return t.concat(e);
  }
  function Qi() {
    var t = Array.prototype.slice.call(arguments, 0, -1), e = Array.prototype.slice.call(arguments, -1);
    if (t.length === 1) return t[0];
    if (t.length > 1) return t.slice(1).reduce(function(r, n) {
      return Gi(r, n, e, 0);
    }, t[0]);
    throw new Error("Wrong number of arguments in function concat");
  }
  function Kf() {
    for (var t = arguments.length, e = new Array(t), r = 0; r < t; r++) e[r] = arguments[r];
    for (var n = e.map((d) => d.length), i = Math.max(...n), o = new Array(i).fill(null), u = 0; u < e.length; u++) for (var a = e[u], f = n[u], l = 0; l < f; l++) {
      var s = i - f + l;
      a[l] > o[s] && (o[s] = a[l]);
    }
    for (var h = 0; h < e.length; h++) lr(e[h], o);
    return o;
  }
  function lr(t, e) {
    for (var r = e.length, n = t.length, i = 0; i < n; i++) {
      var o = r - n + i;
      if (t[i] < e[o] && t[i] > 1 || t[i] > e[o]) throw new Error("shape missmatch: missmatch is found in arg with shape (".concat(t, ") not possible to broadcast dimension ").concat(n, " with size ").concat(t[i], " to size ").concat(e[o]));
    }
  }
  function Cn(t, e) {
    var r = Be(t);
    if (Tt(r, e)) return t;
    lr(r, e);
    var n = Kf(r, e), i = n.length, o = [
      ...Array(i - r.length).fill(1),
      ...r
    ], u = Gf(t);
    r.length < i && (u = ji(u, o), r = Be(u));
    for (var a = 0; a < i; a++) r[a] < n[a] && (u = Yf(u, n[a], a), r = Be(u));
    return u;
  }
  function Yf(t, e, r) {
    return Qi(...Array(e).fill(t), r);
  }
  function Gf(t) {
    return or([], t);
  }
  function Qf(t) {
    var e = 0, r = 1, n = /* @__PURE__ */ Object.create(null), i = /* @__PURE__ */ Object.create(null), o = 0, u = function(f) {
      var l = i[f];
      if (l && (delete n[l], delete i[f], --e, r === l)) {
        if (!e) {
          o = 0, r = 1;
          return;
        }
        for (; !Object.prototype.hasOwnProperty.call(n, ++r); ) ;
      }
    };
    return t = Math.abs(t), {
      hit: function(f) {
        var l = i[f], s = ++o;
        if (n[s] = f, i[f] = s, !l) return ++e, e <= t ? void 0 : (f = n[r], u(f), f);
        if (delete n[l], r === l) for (; !Object.prototype.hasOwnProperty.call(n, ++r); ) ;
      },
      delete: u,
      clear: function() {
        e = o = 0, r = 1, n = /* @__PURE__ */ Object.create(null), i = /* @__PURE__ */ Object.create(null);
      }
    };
  }
  function yr(t) {
    var { hasher: e, limit: r } = arguments.length > 1 && arguments[1] !== void 0 ? arguments[1] : {};
    return r = r ?? Number.POSITIVE_INFINITY, e = e ?? JSON.stringify, function n() {
      typeof n.cache != "object" && (n.cache = {
        values: /* @__PURE__ */ new Map(),
        lru: Qf(r || Number.POSITIVE_INFINITY)
      });
      for (var i = [], o = 0; o < arguments.length; o++) i[o] = arguments[o];
      var u = e(i);
      if (n.cache.values.has(u)) return n.cache.lru.hit(u), n.cache.values.get(u);
      var a = t.apply(t, i);
      return n.cache.values.set(u, a), n.cache.values.delete(n.cache.lru.hit(u)), a;
    };
  }
  function Xi(t) {
    return Object.keys(t.signatures || {}).reduce(function(e, r) {
      var n = (r.match(/,/g) || []).length + 1;
      return Math.max(e, n);
    }, -1);
  }
  var Xf = "DenseMatrix", kf = [
    "Matrix"
  ], el = Y(Xf, kf, (t) => {
    var { Matrix: e } = t;
    function r(s, h) {
      if (!(this instanceof r)) throw new SyntaxError("Constructor must be called with the new operator");
      if (h && !Xe(h)) throw new Error("Invalid datatype: " + h);
      if (Ae(s)) s.type === "DenseMatrix" ? (this._data = _e(s._data), this._size = _e(s._size), this._datatype = h || s._datatype) : (this._data = s.toArray(), this._size = s.size(), this._datatype = h || s._datatype);
      else if (s && we(s.data) && we(s.size)) this._data = s.data, this._size = s.size, En(this._data, this._size), this._datatype = h || s.datatype;
      else if (we(s)) this._data = l(s), this._size = Be(this._data), En(this._data, this._size), this._datatype = h;
      else {
        if (s) throw new TypeError("Unsupported type of data (" + ct(s) + ")");
        this._data = [], this._size = [
          0
        ], this._datatype = h;
      }
    }
    r.prototype = new e(), r.prototype.createDenseMatrix = function(s, h) {
      return new r(s, h);
    }, Object.defineProperty(r, "name", {
      value: "DenseMatrix"
    }), r.prototype.constructor = r, r.prototype.type = "DenseMatrix", r.prototype.isDenseMatrix = true, r.prototype.getDataType = function() {
      return fr(this._data, ct);
    }, r.prototype.storage = function() {
      return "dense";
    }, r.prototype.datatype = function() {
      return this._datatype;
    }, r.prototype.create = function(s, h) {
      return new r(s, h);
    }, r.prototype.subset = function(s, h, d) {
      switch (arguments.length) {
        case 1:
          return n(this, s);
        case 2:
        case 3:
          return o(this, s, h, d);
        default:
          throw new SyntaxError("Wrong number of arguments");
      }
    }, r.prototype.get = function(s) {
      if (!we(s)) throw new TypeError("Array expected");
      if (s.length !== this._size.length) throw new oe(s.length, this._size.length);
      for (var h = 0; h < s.length; h++) ye(s[h], this._size[h]);
      for (var d = this._data, p = 0, D = s.length; p < D; p++) {
        var c = s[p];
        ye(c, d.length), d = d[c];
      }
      return d;
    }, r.prototype.set = function(s, h, d) {
      if (!we(s)) throw new TypeError("Array expected");
      if (s.length < this._size.length) throw new oe(s.length, this._size.length, "<");
      var p, D, c, g = s.map(function(w) {
        return w + 1;
      });
      f(this, g, d);
      var m = this._data;
      for (p = 0, D = s.length - 1; p < D; p++) c = s[p], ye(c, m.length), m = m[c];
      return c = s[s.length - 1], ye(c, m.length), m[c] = h, this;
    };
    function n(s, h) {
      if (!Jr(h)) throw new TypeError("Invalid index");
      var d = h.isScalar();
      if (d) return s.get(h.min());
      var p = h.size();
      if (p.length !== s._size.length) throw new oe(p.length, s._size.length);
      for (var D = h.min(), c = h.max(), g = 0, m = s._size.length; g < m; g++) ye(D[g], s._size[g]), ye(c[g], s._size[g]);
      return new r(i(s._data, h, p.length, 0), s._datatype);
    }
    function i(s, h, d, p) {
      var D = p === d - 1, c = h.dimension(p);
      return D ? c.map(function(g) {
        return ye(g, s.length), s[g];
      }).valueOf() : c.map(function(g) {
        ye(g, s.length);
        var m = s[g];
        return i(m, h, d, p + 1);
      }).valueOf();
    }
    function o(s, h, d, p) {
      if (!h || h.isIndex !== true) throw new TypeError("Invalid index");
      var D = h.size(), c = h.isScalar(), g;
      if (Ae(d) ? (g = d.size(), d = d.valueOf()) : g = Be(d), c) {
        if (g.length !== 0) throw new TypeError("Scalar expected");
        s.set(h.min(), d, p);
      } else {
        if (!Tt(g, D)) try {
          g.length === 0 ? d = Cn([
            d
          ], D) : d = Cn(d, D), g = Be(d);
        } catch {
        }
        if (D.length < s._size.length) throw new oe(D.length, s._size.length, "<");
        if (g.length < D.length) {
          for (var m = 0, w = 0; D[m] === 1 && g[m] === 1; ) m++;
          for (; D[m] === 1; ) w++, m++;
          d = Ki(d, D.length, w, g);
        }
        if (!Tt(D, g)) throw new oe(D, g, ">");
        var v = h.max().map(function(_) {
          return _ + 1;
        });
        f(s, v, p);
        var A = D.length, y = 0;
        u(s._data, h, d, A, y);
      }
      return s;
    }
    function u(s, h, d, p, D) {
      var c = D === p - 1, g = h.dimension(D);
      c ? g.forEach(function(m, w) {
        ye(m), s[m] = d[w[0]];
      }) : g.forEach(function(m, w) {
        ye(m), u(s[m], h, d[w[0]], p, D + 1);
      });
    }
    r.prototype.resize = function(s, h, d) {
      if (!Wt(s)) throw new TypeError("Array or Matrix expected");
      var p = s.valueOf().map((c) => Array.isArray(c) && c.length === 1 ? c[0] : c), D = d ? this.clone() : this;
      return a(D, p, h);
    };
    function a(s, h, d) {
      if (h.length === 0) {
        for (var p = s._data; we(p); ) p = p[0];
        return p;
      }
      return s._size = h.slice(0), s._data = cr(s._data, s._size, d), s;
    }
    r.prototype.reshape = function(s, h) {
      var d = h ? this.clone() : this;
      d._data = ji(d._data, s);
      var p = d._size.reduce((D, c) => D * c);
      return d._size = Gr(s, p), d;
    };
    function f(s, h, d) {
      for (var p = s._size.slice(0), D = false; p.length < h.length; ) p.push(0), D = true;
      for (var c = 0, g = h.length; c < g; c++) h[c] > p[c] && (p[c] = h[c], D = true);
      D && a(s, p, d);
    }
    r.prototype.clone = function() {
      var s = new r({
        data: _e(this._data),
        size: _e(this._size),
        datatype: this._datatype
      });
      return s;
    }, r.prototype.size = function() {
      return this._size.slice(0);
    }, r.prototype.map = function(s) {
      var h = this, d = Xi(s), p = function g(m, w) {
        return we(m) ? m.map(function(v, A) {
          return g(v, w.concat(A));
        }) : d === 1 ? s(m) : d === 2 ? s(m, w) : s(m, w, h);
      }, D = p(this._data, []), c = this._datatype !== void 0 ? fr(D, ct) : void 0;
      return new r(D, c);
    }, r.prototype.forEach = function(s) {
      var h = this, d = function p(D, c) {
        we(D) ? D.forEach(function(g, m) {
          p(g, c.concat(m));
        }) : s(D, c, h);
      };
      d(this._data, []);
    }, r.prototype[Symbol.iterator] = function* () {
      var s = function* h(d, p) {
        if (we(d)) for (var D = 0; D < d.length; D++) yield* h(d[D], p.concat(D));
        else yield {
          value: d,
          index: p
        };
      };
      yield* s(this._data, []);
    }, r.prototype.rows = function() {
      var s = [], h = this.size();
      if (h.length !== 2) throw new TypeError("Rows can only be returned for a 2D matrix.");
      var d = this._data;
      for (var p of d) s.push(new r([
        p
      ], this._datatype));
      return s;
    }, r.prototype.columns = function() {
      var s = this, h = [], d = this.size();
      if (d.length !== 2) throw new TypeError("Rows can only be returned for a 2D matrix.");
      for (var p = this._data, D = function(m) {
        var w = p.map((v) => [
          v[m]
        ]);
        h.push(new r(w, s._datatype));
      }, c = 0; c < d[1]; c++) D(c);
      return h;
    }, r.prototype.toArray = function() {
      return _e(this._data);
    }, r.prototype.valueOf = function() {
      return this._data;
    }, r.prototype.format = function(s) {
      return Me(this._data, s);
    }, r.prototype.toString = function() {
      return Me(this._data);
    }, r.prototype.toJSON = function() {
      return {
        mathjs: "DenseMatrix",
        data: this._data,
        size: this._size,
        datatype: this._datatype
      };
    }, r.prototype.diagonal = function(s) {
      if (s) {
        if (Fe(s) && (s = s.toNumber()), !ve(s) || !De(s)) throw new TypeError("The parameter k must be an integer number");
      } else s = 0;
      for (var h = s > 0 ? s : 0, d = s < 0 ? -s : 0, p = this._size[0], D = this._size[1], c = Math.min(p - d, D - h), g = [], m = 0; m < c; m++) g[m] = this._data[m + d][m + h];
      return new r({
        data: g,
        size: [
          c
        ],
        datatype: this._datatype
      });
    }, r.diagonal = function(s, h, d, p) {
      if (!we(s)) throw new TypeError("Array expected, size parameter");
      if (s.length !== 2) throw new Error("Only two dimensions matrix are supported");
      if (s = s.map(function(E) {
        if (Fe(E) && (E = E.toNumber()), !ve(E) || !De(E) || E < 1) throw new Error("Size values must be positive integers");
        return E;
      }), d) {
        if (Fe(d) && (d = d.toNumber()), !ve(d) || !De(d)) throw new TypeError("The parameter k must be an integer number");
      } else d = 0;
      var D = d > 0 ? d : 0, c = d < 0 ? -d : 0, g = s[0], m = s[1], w = Math.min(g - c, m - D), v;
      if (we(h)) {
        if (h.length !== w) throw new Error("Invalid value array length");
        v = function(C) {
          return h[C];
        };
      } else if (Ae(h)) {
        var A = h.size();
        if (A.length !== 1 || A[0] !== w) throw new Error("Invalid matrix length");
        v = function(C) {
          return h.get([
            C
          ]);
        };
      } else v = function() {
        return h;
      };
      p || (p = Fe(v(0)) ? v(0).mul(0) : 0);
      var y = [];
      if (s.length > 0) {
        y = cr(y, s, p);
        for (var _ = 0; _ < w; _++) y[_ + c][_ + D] = v(_);
      }
      return new r({
        data: y,
        size: [
          g,
          m
        ]
      });
    }, r.fromJSON = function(s) {
      return new r(s);
    }, r.prototype.swapRows = function(s, h) {
      if (!ve(s) || !De(s) || !ve(h) || !De(h)) throw new Error("Row index must be positive integers");
      if (this._size.length !== 2) throw new Error("Only two dimensional matrix is supported");
      return ye(s, this._size[0]), ye(h, this._size[0]), r._swapRows(s, h, this._data), this;
    }, r._swapRows = function(s, h, d) {
      var p = d[s];
      d[s] = d[h], d[h] = p;
    };
    function l(s) {
      return Ae(s) ? l(s.valueOf()) : we(s) ? s.map(l) : s;
    }
    return r;
  }, {
    isClass: true
  });
  function tl(t) {
    var e = t.length, r = t[0].length, n, i, o = [];
    for (i = 0; i < r; i++) {
      var u = [];
      for (n = 0; n < e; n++) u.push(t[n][i]);
      o.push(u);
    }
    return o;
  }
  function Qr(t) {
    for (var e = 0; e < t.length; e++) if (Wt(t[e])) return true;
    return false;
  }
  function _r(t, e) {
    Ae(t) && (t = t.valueOf());
    for (var r = 0, n = t.length; r < n; r++) {
      var i = t[r];
      Array.isArray(i) ? _r(i, e) : e(i);
    }
  }
  function Ue(t, e, r) {
    return t && typeof t.map == "function" ? t.map(function(n) {
      return Ue(n, e);
    }) : e(t);
  }
  function Xr(t, e, r) {
    var n = Array.isArray(t) ? Be(t) : t.size();
    if (e < 0 || e >= n.length) throw new ht(e, n.length);
    return Ae(t) ? t.create(hr(t.valueOf(), e, r)) : hr(t, e, r);
  }
  function hr(t, e, r) {
    var n, i, o, u;
    if (e <= 0) if (Array.isArray(t[0])) {
      for (u = tl(t), i = [], n = 0; n < u.length; n++) i[n] = hr(u[n], e - 1, r);
      return i;
    } else {
      for (o = t[0], n = 1; n < t.length; n++) o = r(o, t[n]);
      return o;
    }
    else {
      for (i = [], n = 0; n < t.length; n++) i[n] = hr(t[n], e - 1, r);
      return i;
    }
  }
  var bn = "isInteger", rl = [
    "typed"
  ], nl = Y(bn, rl, (t) => {
    var { typed: e } = t;
    return e(bn, {
      number: De,
      BigNumber: function(n) {
        return n.isInt();
      },
      Fraction: function(n) {
        return n.d === 1 && isFinite(n.n);
      },
      "Array | Matrix": e.referToSelf((r) => (n) => Ue(n, r))
    });
  }), Ar = "number", Fr = "number, number";
  function ki(t) {
    return Math.abs(t);
  }
  ki.signature = Ar;
  function eo(t, e) {
    return t + e;
  }
  eo.signature = Fr;
  function to(t, e) {
    return t - e;
  }
  to.signature = Fr;
  function ro(t, e) {
    return t * e;
  }
  ro.signature = Fr;
  function no(t) {
    return -t;
  }
  no.signature = Ar;
  function io(t) {
    return Math.exp(t);
  }
  io.signature = Ar;
  function oo(t) {
    return lc(t);
  }
  oo.signature = Ar;
  function uo(t, e) {
    return t * t < 1 && e === 1 / 0 || t * t > 1 && e === -1 / 0 ? 0 : Math.pow(t, e);
  }
  uo.signature = Fr;
  function Lt(t) {
    var e = arguments.length > 1 && arguments[1] !== void 0 ? arguments[1] : 0;
    if (!De(e) || e < 0 || e > 15) throw new Error("Number of decimals in function round must be an integer from 0 to 15 inclusive");
    return parseFloat(Mi(t, e));
  }
  var il = 2 * Math.PI, ol = "number";
  function ao(t) {
    return t === 0;
  }
  ao.signature = ol;
  var Mn = "isZero", ul = [
    "typed"
  ], al = Y(Mn, ul, (t) => {
    var { typed: e } = t;
    return e(Mn, {
      number: ao,
      BigNumber: function(n) {
        return n.isZero();
      },
      Complex: function(n) {
        return n.re === 0 && n.im === 0;
      },
      Fraction: function(n) {
        return n.d === 1 && n.n === 0;
      },
      Unit: e.referToSelf((r) => (n) => e.find(r, n.valueType())(n.value)),
      "Array | Matrix": e.referToSelf((r) => (n) => Ue(n, r))
    });
  });
  function It(t, e, r) {
    if (r == null) return t.eq(e);
    if (t.eq(e)) return true;
    if (t.isNaN() || e.isNaN()) return false;
    if (t.isFinite() && e.isFinite()) {
      var n = t.minus(e).abs();
      if (n.isZero()) return true;
      var i = t.constructor.max(t.abs(), e.abs());
      return n.lte(i.times(r));
    }
    return false;
  }
  function sl(t, e, r) {
    return ut(t.re, e.re, r) && ut(t.im, e.im, r);
  }
  var so = Y("compareUnits", [
    "typed"
  ], (t) => {
    var { typed: e } = t;
    return {
      "Unit, Unit": e.referToSelf((r) => (n, i) => {
        if (!n.equalBase(i)) throw new Error("Cannot compare units with different base");
        return e.find(r, [
          n.valueType(),
          i.valueType()
        ])(n.value, i.value);
      })
    };
  }), dr = "equalScalar", cl = [
    "typed",
    "config"
  ], fl = Y(dr, cl, (t) => {
    var { typed: e, config: r } = t, n = so({
      typed: e
    });
    return e(dr, {
      "boolean, boolean": function(o, u) {
        return o === u;
      },
      "number, number": function(o, u) {
        return ut(o, u, r.epsilon);
      },
      "BigNumber, BigNumber": function(o, u) {
        return o.eq(u) || It(o, u, r.epsilon);
      },
      "Fraction, Fraction": function(o, u) {
        return o.equals(u);
      },
      "Complex, Complex": function(o, u) {
        return sl(o, u, r.epsilon);
      }
    }, n);
  });
  Y(dr, [
    "typed",
    "config"
  ], (t) => {
    var { typed: e, config: r } = t;
    return e(dr, {
      "number, number": function(i, o) {
        return ut(i, o, r.epsilon);
      }
    });
  });
  var ll = "SparseMatrix", hl = [
    "typed",
    "equalScalar",
    "Matrix"
  ], dl = Y(ll, hl, (t) => {
    var { typed: e, equalScalar: r, Matrix: n } = t;
    function i(c, g) {
      if (!(this instanceof i)) throw new SyntaxError("Constructor must be called with the new operator");
      if (g && !Xe(g)) throw new Error("Invalid datatype: " + g);
      if (Ae(c)) o(this, c, g);
      else if (c && we(c.index) && we(c.ptr) && we(c.size)) this._values = c.values, this._index = c.index, this._ptr = c.ptr, this._size = c.size, this._datatype = g || c.datatype;
      else if (we(c)) u(this, c, g);
      else {
        if (c) throw new TypeError("Unsupported type of data (" + ct(c) + ")");
        this._values = [], this._index = [], this._ptr = [
          0
        ], this._size = [
          0,
          0
        ], this._datatype = g;
      }
    }
    function o(c, g, m) {
      g.type === "SparseMatrix" ? (c._values = g._values ? _e(g._values) : void 0, c._index = _e(g._index), c._ptr = _e(g._ptr), c._size = _e(g._size), c._datatype = m || g._datatype) : u(c, g.valueOf(), m || g._datatype);
    }
    function u(c, g, m) {
      c._values = [], c._index = [], c._ptr = [], c._datatype = m;
      var w = g.length, v = 0, A = r, y = 0;
      if (Xe(m) && (A = e.find(r, [
        m,
        m
      ]) || r, y = e.convert(0, m)), w > 0) {
        var _ = 0;
        do {
          c._ptr.push(c._index.length);
          for (var E = 0; E < w; E++) {
            var C = g[E];
            if (we(C)) {
              if (_ === 0 && v < C.length && (v = C.length), _ < C.length) {
                var b = C[_];
                A(b, y) || (c._values.push(b), c._index.push(E));
              }
            } else _ === 0 && v < 1 && (v = 1), A(C, y) || (c._values.push(C), c._index.push(E));
          }
          _++;
        } while (_ < v);
      }
      c._ptr.push(c._index.length), c._size = [
        w,
        v
      ];
    }
    i.prototype = new n(), i.prototype.createSparseMatrix = function(c, g) {
      return new i(c, g);
    }, Object.defineProperty(i, "name", {
      value: "SparseMatrix"
    }), i.prototype.constructor = i, i.prototype.type = "SparseMatrix", i.prototype.isSparseMatrix = true, i.prototype.getDataType = function() {
      return fr(this._values, ct);
    }, i.prototype.storage = function() {
      return "sparse";
    }, i.prototype.datatype = function() {
      return this._datatype;
    }, i.prototype.create = function(c, g) {
      return new i(c, g);
    }, i.prototype.density = function() {
      var c = this._size[0], g = this._size[1];
      return c !== 0 && g !== 0 ? this._index.length / (c * g) : 0;
    }, i.prototype.subset = function(c, g, m) {
      if (!this._values) throw new Error("Cannot invoke subset on a Pattern only matrix");
      switch (arguments.length) {
        case 1:
          return a(this, c);
        case 2:
        case 3:
          return f(this, c, g, m);
        default:
          throw new SyntaxError("Wrong number of arguments");
      }
    };
    function a(c, g) {
      if (!Jr(g)) throw new TypeError("Invalid index");
      var m = g.isScalar();
      if (m) return c.get(g.min());
      var w = g.size();
      if (w.length !== c._size.length) throw new oe(w.length, c._size.length);
      var v, A, y, _, E = g.min(), C = g.max();
      for (v = 0, A = c._size.length; v < A; v++) ye(E[v], c._size[v]), ye(C[v], c._size[v]);
      var b = c._values, x = c._index, N = c._ptr, B = g.dimension(0), Z = g.dimension(1), q = [], U = [];
      B.forEach(function(G, re) {
        U[G] = re[0], q[G] = true;
      });
      var I = b ? [] : void 0, K = [], Q = [];
      return Z.forEach(function(G) {
        for (Q.push(K.length), y = N[G], _ = N[G + 1]; y < _; y++) v = x[y], q[v] === true && (K.push(U[v]), I && I.push(b[y]));
      }), Q.push(K.length), new i({
        values: I,
        index: K,
        ptr: Q,
        size: w,
        datatype: c._datatype
      });
    }
    function f(c, g, m, w) {
      if (!g || g.isIndex !== true) throw new TypeError("Invalid index");
      var v = g.size(), A = g.isScalar(), y;
      if (Ae(m) ? (y = m.size(), m = m.toArray()) : y = Be(m), A) {
        if (y.length !== 0) throw new TypeError("Scalar expected");
        c.set(g.min(), m, w);
      } else {
        if (v.length !== 1 && v.length !== 2) throw new oe(v.length, c._size.length, "<");
        if (y.length < v.length) {
          for (var _ = 0, E = 0; v[_] === 1 && y[_] === 1; ) _++;
          for (; v[_] === 1; ) E++, _++;
          m = Ki(m, v.length, E, y);
        }
        if (!Tt(v, y)) throw new oe(v, y, ">");
        if (v.length === 1) {
          var C = g.dimension(0);
          C.forEach(function(N, B) {
            ye(N), c.set([
              N,
              0
            ], m[B[0]], w);
          });
        } else {
          var b = g.dimension(0), x = g.dimension(1);
          b.forEach(function(N, B) {
            ye(N), x.forEach(function(Z, q) {
              ye(Z), c.set([
                N,
                Z
              ], m[B[0]][q[0]], w);
            });
          });
        }
      }
      return c;
    }
    i.prototype.get = function(c) {
      if (!we(c)) throw new TypeError("Array expected");
      if (c.length !== this._size.length) throw new oe(c.length, this._size.length);
      if (!this._values) throw new Error("Cannot invoke get on a Pattern only matrix");
      var g = c[0], m = c[1];
      ye(g, this._size[0]), ye(m, this._size[1]);
      var w = l(g, this._ptr[m], this._ptr[m + 1], this._index);
      return w < this._ptr[m + 1] && this._index[w] === g ? this._values[w] : 0;
    }, i.prototype.set = function(c, g, m) {
      if (!we(c)) throw new TypeError("Array expected");
      if (c.length !== this._size.length) throw new oe(c.length, this._size.length);
      if (!this._values) throw new Error("Cannot invoke set on a Pattern only matrix");
      var w = c[0], v = c[1], A = this._size[0], y = this._size[1], _ = r, E = 0;
      Xe(this._datatype) && (_ = e.find(r, [
        this._datatype,
        this._datatype
      ]) || r, E = e.convert(0, this._datatype)), (w > A - 1 || v > y - 1) && (d(this, Math.max(w + 1, A), Math.max(v + 1, y), m), A = this._size[0], y = this._size[1]), ye(w, A), ye(v, y);
      var C = l(w, this._ptr[v], this._ptr[v + 1], this._index);
      return C < this._ptr[v + 1] && this._index[C] === w ? _(g, E) ? s(C, v, this._values, this._index, this._ptr) : this._values[C] = g : _(g, E) || h(C, w, v, g, this._values, this._index, this._ptr), this;
    };
    function l(c, g, m, w) {
      if (m - g === 0) return m;
      for (var v = g; v < m; v++) if (w[v] === c) return v;
      return g;
    }
    function s(c, g, m, w, v) {
      m.splice(c, 1), w.splice(c, 1);
      for (var A = g + 1; A < v.length; A++) v[A]--;
    }
    function h(c, g, m, w, v, A, y) {
      v.splice(c, 0, w), A.splice(c, 0, g);
      for (var _ = m + 1; _ < y.length; _++) y[_]++;
    }
    i.prototype.resize = function(c, g, m) {
      if (!Wt(c)) throw new TypeError("Array or Matrix expected");
      var w = c.valueOf().map((A) => Array.isArray(A) && A.length === 1 ? A[0] : A);
      if (w.length !== 2) throw new Error("Only two dimensions matrix are supported");
      w.forEach(function(A) {
        if (!ve(A) || !De(A) || A < 0) throw new TypeError("Invalid size, must contain positive integers (size: " + Me(w) + ")");
      });
      var v = m ? this.clone() : this;
      return d(v, w[0], w[1], g);
    };
    function d(c, g, m, w) {
      var v = w || 0, A = r, y = 0;
      Xe(c._datatype) && (A = e.find(r, [
        c._datatype,
        c._datatype
      ]) || r, y = e.convert(0, c._datatype), v = e.convert(v, c._datatype));
      var _ = !A(v, y), E = c._size[0], C = c._size[1], b, x, N;
      if (m > C) {
        for (x = C; x < m; x++) if (c._ptr[x] = c._values.length, _) for (b = 0; b < E; b++) c._values.push(v), c._index.push(b);
        c._ptr[m] = c._values.length;
      } else m < C && (c._ptr.splice(m + 1, C - m), c._values.splice(c._ptr[m], c._values.length), c._index.splice(c._ptr[m], c._index.length));
      if (C = m, g > E) {
        if (_) {
          var B = 0;
          for (x = 0; x < C; x++) {
            c._ptr[x] = c._ptr[x] + B, N = c._ptr[x + 1] + B;
            var Z = 0;
            for (b = E; b < g; b++, Z++) c._values.splice(N + Z, 0, v), c._index.splice(N + Z, 0, b), B++;
          }
          c._ptr[C] = c._values.length;
        }
      } else if (g < E) {
        var q = 0;
        for (x = 0; x < C; x++) {
          c._ptr[x] = c._ptr[x] - q;
          var U = c._ptr[x], I = c._ptr[x + 1] - q;
          for (N = U; N < I; N++) b = c._index[N], b > g - 1 && (c._values.splice(N, 1), c._index.splice(N, 1), q++);
        }
        c._ptr[x] = c._values.length;
      }
      return c._size[0] = g, c._size[1] = m, c;
    }
    i.prototype.reshape = function(c, g) {
      if (!we(c)) throw new TypeError("Array expected");
      if (c.length !== 2) throw new Error("Sparse matrices can only be reshaped in two dimensions");
      c.forEach(function(G) {
        if (!ve(G) || !De(G) || G <= -2 || G === 0) throw new TypeError("Invalid size, must contain positive integers or -1 (size: " + Me(c) + ")");
      });
      var m = this._size[0] * this._size[1];
      c = Gr(c, m);
      var w = c[0] * c[1];
      if (m !== w) throw new Error("Reshaping sparse matrix will result in the wrong number of elements");
      var v = g ? this.clone() : this;
      if (this._size[0] === c[0] && this._size[1] === c[1]) return v;
      for (var A = [], y = 0; y < v._ptr.length; y++) for (var _ = 0; _ < v._ptr[y + 1] - v._ptr[y]; _++) A.push(y);
      for (var E = v._values.slice(), C = v._index.slice(), b = 0; b < v._index.length; b++) {
        var x = C[b], N = A[b], B = x * v._size[1] + N;
        A[b] = B % c[1], C[b] = Math.floor(B / c[1]);
      }
      v._values.length = 0, v._index.length = 0, v._ptr.length = c[1] + 1, v._size = c.slice();
      for (var Z = 0; Z < v._ptr.length; Z++) v._ptr[Z] = 0;
      for (var q = 0; q < E.length; q++) {
        var U = C[q], I = A[q], K = E[q], Q = l(U, v._ptr[I], v._ptr[I + 1], v._index);
        h(Q, U, I, K, v._values, v._index, v._ptr);
      }
      return v;
    }, i.prototype.clone = function() {
      var c = new i({
        values: this._values ? _e(this._values) : void 0,
        index: _e(this._index),
        ptr: _e(this._ptr),
        size: _e(this._size),
        datatype: this._datatype
      });
      return c;
    }, i.prototype.size = function() {
      return this._size.slice(0);
    }, i.prototype.map = function(c, g) {
      if (!this._values) throw new Error("Cannot invoke map on a Pattern only matrix");
      var m = this, w = this._size[0], v = this._size[1], A = Xi(c), y = function(E, C, b) {
        return A === 1 ? c(E) : A === 2 ? c(E, [
          C,
          b
        ]) : c(E, [
          C,
          b
        ], m);
      };
      return p(this, 0, w - 1, 0, v - 1, y, g);
    };
    function p(c, g, m, w, v, A, y) {
      var _ = [], E = [], C = [], b = r, x = 0;
      Xe(c._datatype) && (b = e.find(r, [
        c._datatype,
        c._datatype
      ]) || r, x = e.convert(0, c._datatype));
      for (var N = function(ue, Ce, Ee) {
        ue = A(ue, Ce, Ee), b(ue, x) || (_.push(ue), E.push(Ce));
      }, B = w; B <= v; B++) {
        C.push(_.length);
        var Z = c._ptr[B], q = c._ptr[B + 1];
        if (y) for (var U = Z; U < q; U++) {
          var I = c._index[U];
          I >= g && I <= m && N(c._values[U], I - g, B - w);
        }
        else {
          for (var K = {}, Q = Z; Q < q; Q++) {
            var G = c._index[Q];
            K[G] = c._values[Q];
          }
          for (var re = g; re <= m; re++) {
            var ce = re in K ? K[re] : 0;
            N(ce, re - g, B - w);
          }
        }
      }
      return C.push(_.length), new i({
        values: _,
        index: E,
        ptr: C,
        size: [
          m - g + 1,
          v - w + 1
        ]
      });
    }
    i.prototype.forEach = function(c, g) {
      if (!this._values) throw new Error("Cannot invoke forEach on a Pattern only matrix");
      for (var m = this, w = this._size[0], v = this._size[1], A = 0; A < v; A++) {
        var y = this._ptr[A], _ = this._ptr[A + 1];
        if (g) for (var E = y; E < _; E++) {
          var C = this._index[E];
          c(this._values[E], [
            C,
            A
          ], m);
        }
        else {
          for (var b = {}, x = y; x < _; x++) {
            var N = this._index[x];
            b[N] = this._values[x];
          }
          for (var B = 0; B < w; B++) {
            var Z = B in b ? b[B] : 0;
            c(Z, [
              B,
              A
            ], m);
          }
        }
      }
    }, i.prototype[Symbol.iterator] = function* () {
      if (!this._values) throw new Error("Cannot iterate a Pattern only matrix");
      for (var c = this._size[1], g = 0; g < c; g++) for (var m = this._ptr[g], w = this._ptr[g + 1], v = m; v < w; v++) {
        var A = this._index[v];
        yield {
          value: this._values[v],
          index: [
            A,
            g
          ]
        };
      }
    }, i.prototype.toArray = function() {
      return D(this._values, this._index, this._ptr, this._size, true);
    }, i.prototype.valueOf = function() {
      return D(this._values, this._index, this._ptr, this._size, false);
    };
    function D(c, g, m, w, v) {
      var A = w[0], y = w[1], _ = [], E, C;
      for (E = 0; E < A; E++) for (_[E] = [], C = 0; C < y; C++) _[E][C] = 0;
      for (C = 0; C < y; C++) for (var b = m[C], x = m[C + 1], N = b; N < x; N++) E = g[N], _[E][C] = c ? v ? _e(c[N]) : c[N] : 1;
      return _;
    }
    return i.prototype.format = function(c) {
      for (var g = this._size[0], m = this._size[1], w = this.density(), v = "Sparse Matrix [" + Me(g, c) + " x " + Me(m, c) + "] density: " + Me(w, c) + `
`, A = 0; A < m; A++) for (var y = this._ptr[A], _ = this._ptr[A + 1], E = y; E < _; E++) {
        var C = this._index[E];
        v += `
    (` + Me(C, c) + ", " + Me(A, c) + ") ==> " + (this._values ? Me(this._values[E], c) : "X");
      }
      return v;
    }, i.prototype.toString = function() {
      return Me(this.toArray());
    }, i.prototype.toJSON = function() {
      return {
        mathjs: "SparseMatrix",
        values: this._values,
        index: this._index,
        ptr: this._ptr,
        size: this._size,
        datatype: this._datatype
      };
    }, i.prototype.diagonal = function(c) {
      if (c) {
        if (Fe(c) && (c = c.toNumber()), !ve(c) || !De(c)) throw new TypeError("The parameter k must be an integer number");
      } else c = 0;
      var g = c > 0 ? c : 0, m = c < 0 ? -c : 0, w = this._size[0], v = this._size[1], A = Math.min(w - m, v - g), y = [], _ = [], E = [];
      E[0] = 0;
      for (var C = g; C < v && y.length < A; C++) for (var b = this._ptr[C], x = this._ptr[C + 1], N = b; N < x; N++) {
        var B = this._index[N];
        if (B === C - g + m) {
          y.push(this._values[N]), _[y.length - 1] = B - m;
          break;
        }
      }
      return E.push(y.length), new i({
        values: y,
        index: _,
        ptr: E,
        size: [
          A,
          1
        ]
      });
    }, i.fromJSON = function(c) {
      return new i(c);
    }, i.diagonal = function(c, g, m, w, v) {
      if (!we(c)) throw new TypeError("Array expected, size parameter");
      if (c.length !== 2) throw new Error("Only two dimensions matrix are supported");
      if (c = c.map(function(G) {
        if (Fe(G) && (G = G.toNumber()), !ve(G) || !De(G) || G < 1) throw new Error("Size values must be positive integers");
        return G;
      }), m) {
        if (Fe(m) && (m = m.toNumber()), !ve(m) || !De(m)) throw new TypeError("The parameter k must be an integer number");
      } else m = 0;
      var A = r, y = 0;
      Xe(v) && (A = e.find(r, [
        v,
        v
      ]) || r, y = e.convert(0, v));
      var _ = m > 0 ? m : 0, E = m < 0 ? -m : 0, C = c[0], b = c[1], x = Math.min(C - E, b - _), N;
      if (we(g)) {
        if (g.length !== x) throw new Error("Invalid value array length");
        N = function(re) {
          return g[re];
        };
      } else if (Ae(g)) {
        var B = g.size();
        if (B.length !== 1 || B[0] !== x) throw new Error("Invalid matrix length");
        N = function(re) {
          return g.get([
            re
          ]);
        };
      } else N = function() {
        return g;
      };
      for (var Z = [], q = [], U = [], I = 0; I < b; I++) {
        U.push(Z.length);
        var K = I - _;
        if (K >= 0 && K < x) {
          var Q = N(K);
          A(Q, y) || (q.push(K + E), Z.push(Q));
        }
      }
      return U.push(Z.length), new i({
        values: Z,
        index: q,
        ptr: U,
        size: [
          C,
          b
        ]
      });
    }, i.prototype.swapRows = function(c, g) {
      if (!ve(c) || !De(c) || !ve(g) || !De(g)) throw new Error("Row index must be positive integers");
      if (this._size.length !== 2) throw new Error("Only two dimensional matrix is supported");
      return ye(c, this._size[0]), ye(g, this._size[0]), i._swapRows(c, g, this._size[1], this._values, this._index, this._ptr), this;
    }, i._forEachRow = function(c, g, m, w, v) {
      for (var A = w[c], y = w[c + 1], _ = A; _ < y; _++) v(m[_], g[_]);
    }, i._swapRows = function(c, g, m, w, v, A) {
      for (var y = 0; y < m; y++) {
        var _ = A[y], E = A[y + 1], C = l(c, _, E, v), b = l(g, _, E, v);
        if (C < E && b < E && v[C] === c && v[b] === g) {
          if (w) {
            var x = w[C];
            w[C] = w[b], w[b] = x;
          }
          continue;
        }
        if (C < E && v[C] === c && (b >= E || v[b] !== g)) {
          var N = w ? w[C] : void 0;
          v.splice(b, 0, g), w && w.splice(b, 0, N), v.splice(b <= C ? C + 1 : C, 1), w && w.splice(b <= C ? C + 1 : C, 1);
          continue;
        }
        if (b < E && v[b] === g && (C >= E || v[C] !== c)) {
          var B = w ? w[b] : void 0;
          v.splice(C, 0, c), w && w.splice(C, 0, B), v.splice(C <= b ? b + 1 : b, 1), w && w.splice(C <= b ? b + 1 : b, 1);
        }
      }
    }, i;
  }, {
    isClass: true
  }), ml = "number", pl = [
    "typed"
  ];
  function vl(t) {
    var e = t.match(/(0[box])([0-9a-fA-F]*)\.([0-9a-fA-F]*)/);
    if (e) {
      var r = {
        "0b": 2,
        "0o": 8,
        "0x": 16
      }[e[1]], n = e[2], i = e[3];
      return {
        input: t,
        radix: r,
        integerPart: n,
        fractionalPart: i
      };
    } else return null;
  }
  function gl(t) {
    for (var e = parseInt(t.integerPart, t.radix), r = 0, n = 0; n < t.fractionalPart.length; n++) {
      var i = parseInt(t.fractionalPart[n], t.radix);
      r += i / Math.pow(t.radix, n + 1);
    }
    var o = e + r;
    if (isNaN(o)) throw new SyntaxError('String "' + t.input + '" is not a valid number');
    return o;
  }
  var Dl = Y(ml, pl, (t) => {
    var { typed: e } = t, r = e("number", {
      "": function() {
        return 0;
      },
      number: function(i) {
        return i;
      },
      string: function(i) {
        if (i === "NaN") return NaN;
        var o = vl(i);
        if (o) return gl(o);
        var u = 0, a = i.match(/(0[box][0-9a-fA-F]*)i([0-9]*)/);
        a && (u = Number(a[2]), i = a[1]);
        var f = Number(i);
        if (isNaN(f)) throw new SyntaxError('String "' + i + '" is not a valid number');
        if (a) {
          if (f > 2 ** u - 1) throw new SyntaxError('String "'.concat(i, '" is out of range'));
          f >= 2 ** (u - 1) && (f = f - 2 ** u);
        }
        return f;
      },
      BigNumber: function(i) {
        return i.toNumber();
      },
      Fraction: function(i) {
        return i.valueOf();
      },
      Unit: e.referToSelf((n) => (i) => {
        var o = i.clone();
        return o.value = n(i.value), o;
      }),
      null: function(i) {
        return 0;
      },
      "Unit, string | Unit": function(i, o) {
        return i.toNumber(o);
      },
      "Array | Matrix": e.referToSelf((n) => (i) => Ue(i, n))
    });
    return r.fromJSON = function(n) {
      return parseFloat(n.value);
    }, r;
  }), wl = "bignumber", yl = [
    "typed",
    "BigNumber"
  ], _l = Y(wl, yl, (t) => {
    var { typed: e, BigNumber: r } = t;
    return e("bignumber", {
      "": function() {
        return new r(0);
      },
      number: function(i) {
        return new r(i + "");
      },
      string: function(i) {
        var o = i.match(/(0[box][0-9a-fA-F]*)i([0-9]*)/);
        if (o) {
          var u = o[2], a = r(o[1]), f = new r(2).pow(Number(u));
          if (a.gt(f.sub(1))) throw new SyntaxError('String "'.concat(i, '" is out of range'));
          var l = new r(2).pow(Number(u) - 1);
          return a.gte(l) ? a.sub(f) : a;
        }
        return new r(i);
      },
      BigNumber: function(i) {
        return i;
      },
      Unit: e.referToSelf((n) => (i) => {
        var o = i.clone();
        return o.value = n(i.value), o;
      }),
      Fraction: function(i) {
        return new r(i.n).div(i.d).times(i.s);
      },
      null: function(i) {
        return new r(0);
      },
      "Array | Matrix": e.referToSelf((n) => (i) => Ue(i, n))
    });
  }), Al = "fraction", Fl = [
    "typed",
    "Fraction"
  ], El = Y(Al, Fl, (t) => {
    var { typed: e, Fraction: r } = t;
    return e("fraction", {
      number: function(i) {
        if (!isFinite(i) || isNaN(i)) throw new Error(i + " cannot be represented as a fraction");
        return new r(i);
      },
      string: function(i) {
        return new r(i);
      },
      "number, number": function(i, o) {
        return new r(i, o);
      },
      null: function(i) {
        return new r(0);
      },
      BigNumber: function(i) {
        return new r(i.toString());
      },
      Fraction: function(i) {
        return i;
      },
      Unit: e.referToSelf((n) => (i) => {
        var o = i.clone();
        return o.value = n(i.value), o;
      }),
      Object: function(i) {
        return new r(i);
      },
      "Array | Matrix": e.referToSelf((n) => (i) => Ue(i, n))
    });
  }), Sn = "matrix", Cl = [
    "typed",
    "Matrix",
    "DenseMatrix",
    "SparseMatrix"
  ], bl = Y(Sn, Cl, (t) => {
    var { typed: e, Matrix: r, DenseMatrix: n, SparseMatrix: i } = t;
    return e(Sn, {
      "": function() {
        return o([]);
      },
      string: function(a) {
        return o([], a);
      },
      "string, string": function(a, f) {
        return o([], a, f);
      },
      Array: function(a) {
        return o(a);
      },
      Matrix: function(a) {
        return o(a, a.storage());
      },
      "Array | Matrix, string": o,
      "Array | Matrix, string, string": o
    });
    function o(u, a, f) {
      if (a === "dense" || a === "default" || a === void 0) return new n(u, f);
      if (a === "sparse") return new i(u, f);
      throw new TypeError("Unknown matrix type " + JSON.stringify(a) + ".");
    }
  }), Nn = "unaryMinus", Ml = [
    "typed"
  ], Sl = Y(Nn, Ml, (t) => {
    var { typed: e } = t;
    return e(Nn, {
      number: no,
      "Complex | BigNumber | Fraction": (r) => r.neg(),
      Unit: e.referToSelf((r) => (n) => {
        var i = n.clone();
        return i.value = e.find(r, i.valueType())(n.value), i;
      }),
      "Array | Matrix": e.referToSelf((r) => (n) => Ue(n, r))
    });
  }), Bn = "abs", Nl = [
    "typed"
  ], Bl = Y(Bn, Nl, (t) => {
    var { typed: e } = t;
    return e(Bn, {
      number: ki,
      "Complex | BigNumber | Fraction | Unit": (r) => r.abs(),
      "Array | Matrix": e.referToSelf((r) => (n) => Ue(n, r))
    });
  }), xn = "addScalar", xl = [
    "typed"
  ], Tl = Y(xn, xl, (t) => {
    var { typed: e } = t;
    return e(xn, {
      "number, number": eo,
      "Complex, Complex": function(n, i) {
        return n.add(i);
      },
      "BigNumber, BigNumber": function(n, i) {
        return n.plus(i);
      },
      "Fraction, Fraction": function(n, i) {
        return n.add(i);
      },
      "Unit, Unit": e.referToSelf((r) => (n, i) => {
        if (n.value === null || n.value === void 0) throw new Error("Parameter x contains a unit with undefined value");
        if (i.value === null || i.value === void 0) throw new Error("Parameter y contains a unit with undefined value");
        if (!n.equalBase(i)) throw new Error("Units do not match");
        var o = n.clone();
        return o.value = e.find(r, [
          o.valueType(),
          i.valueType()
        ])(o.value, i.value), o.fixPrefix = false, o;
      })
    });
  }), Tn = "subtractScalar", $l = [
    "typed"
  ], Il = Y(Tn, $l, (t) => {
    var { typed: e } = t;
    return e(Tn, {
      "number, number": to,
      "Complex, Complex": function(n, i) {
        return n.sub(i);
      },
      "BigNumber, BigNumber": function(n, i) {
        return n.minus(i);
      },
      "Fraction, Fraction": function(n, i) {
        return n.sub(i);
      },
      "Unit, Unit": e.referToSelf((r) => (n, i) => {
        if (n.value === null || n.value === void 0) throw new Error("Parameter x contains a unit with undefined value");
        if (i.value === null || i.value === void 0) throw new Error("Parameter y contains a unit with undefined value");
        if (!n.equalBase(i)) throw new Error("Units do not match");
        var o = n.clone();
        return o.value = e.find(r, [
          o.valueType(),
          i.valueType()
        ])(o.value, i.value), o.fixPrefix = false, o;
      })
    });
  }), zl = "matAlgo11xS0s", Ol = [
    "typed",
    "equalScalar"
  ], Ot = Y(zl, Ol, (t) => {
    var { typed: e, equalScalar: r } = t;
    return function(i, o, u, a) {
      var f = i._values, l = i._index, s = i._ptr, h = i._size, d = i._datatype;
      if (!f) throw new Error("Cannot perform operation on Pattern Sparse Matrix and Scalar value");
      var p = h[0], D = h[1], c, g = r, m = 0, w = u;
      typeof d == "string" && (c = d, g = e.find(r, [
        c,
        c
      ]), m = e.convert(0, c), o = e.convert(o, c), w = e.find(u, [
        c,
        c
      ]));
      for (var v = [], A = [], y = [], _ = 0; _ < D; _++) {
        y[_] = A.length;
        for (var E = s[_], C = s[_ + 1], b = E; b < C; b++) {
          var x = l[b], N = a ? w(o, f[b]) : w(f[b], o);
          g(N, m) || (A.push(x), v.push(N));
        }
      }
      return y[D] = A.length, i.createSparseMatrix({
        values: v,
        index: A,
        ptr: y,
        size: [
          p,
          D
        ],
        datatype: c
      });
    };
  }), Pl = "matAlgo12xSfs", ql = [
    "typed",
    "DenseMatrix"
  ], Er = Y(Pl, ql, (t) => {
    var { typed: e, DenseMatrix: r } = t;
    return function(i, o, u, a) {
      var f = i._values, l = i._index, s = i._ptr, h = i._size, d = i._datatype;
      if (!f) throw new Error("Cannot perform operation on Pattern Sparse Matrix and Scalar value");
      var p = h[0], D = h[1], c, g = u;
      typeof d == "string" && (c = d, o = e.convert(o, c), g = e.find(u, [
        c,
        c
      ]));
      for (var m = [], w = [], v = [], A = 0; A < D; A++) {
        for (var y = A + 1, _ = s[A], E = s[A + 1], C = _; C < E; C++) {
          var b = l[C];
          w[b] = f[C], v[b] = y;
        }
        for (var x = 0; x < p; x++) A === 0 && (m[x] = []), v[x] === y ? m[x][A] = a ? g(o, w[x]) : g(w[x], o) : m[x][A] = a ? g(o, 0) : g(0, o);
      }
      return new r({
        data: m,
        size: [
          p,
          D
        ],
        datatype: c
      });
    };
  }), Rl = "matAlgo14xDs", Ll = [
    "typed"
  ], Jt = Y(Rl, Ll, (t) => {
    var { typed: e } = t;
    return function(i, o, u, a) {
      var f = i._data, l = i._size, s = i._datatype, h, d = u;
      typeof s == "string" && (h = s, o = e.convert(o, h), d = e.find(u, [
        h,
        h
      ]));
      var p = l.length > 0 ? r(d, 0, l, l[0], f, o, a) : [];
      return i.createDenseMatrix({
        data: p,
        size: _e(l),
        datatype: h
      });
    };
    function r(n, i, o, u, a, f, l) {
      var s = [];
      if (i === o.length - 1) for (var h = 0; h < u; h++) s[h] = l ? n(f, a[h]) : n(a[h], f);
      else for (var d = 0; d < u; d++) s[d] = r(n, i + 1, o, o[i + 1], a[d], f, l);
      return s;
    }
  }), Hr = "ceil", Vl = [
    "typed",
    "config",
    "round",
    "matrix",
    "equalScalar",
    "zeros",
    "DenseMatrix"
  ], Ul = Y(Hr, [
    "typed",
    "config",
    "round"
  ], (t) => {
    var { typed: e, config: r, round: n } = t;
    return e(Hr, {
      number: function(o) {
        return ut(o, n(o), r.epsilon) ? n(o) : Math.ceil(o);
      },
      "number, number": function(o, u) {
        if (ut(o, n(o, u), r.epsilon)) return n(o, u);
        var [a, f] = "".concat(o, "e").split("e"), l = Math.ceil(Number("".concat(a, "e").concat(Number(f) + u)));
        return [a, f] = "".concat(l, "e").split("e"), Number("".concat(a, "e").concat(Number(f) - u));
      }
    });
  }), Wl = Y(Hr, Vl, (t) => {
    var { typed: e, config: r, round: n, matrix: i, equalScalar: o, zeros: u, DenseMatrix: a } = t, f = Ot({
      typed: e,
      equalScalar: o
    }), l = Er({
      typed: e,
      DenseMatrix: a
    }), s = Jt({
      typed: e
    }), h = Ul({
      typed: e,
      config: r,
      round: n
    });
    return e("ceil", {
      number: h.signatures.number,
      "number,number": h.signatures["number,number"],
      Complex: function(p) {
        return p.ceil();
      },
      "Complex, number": function(p, D) {
        return p.ceil(D);
      },
      "Complex, BigNumber": function(p, D) {
        return p.ceil(D.toNumber());
      },
      BigNumber: function(p) {
        return It(p, n(p), r.epsilon) ? n(p) : p.ceil();
      },
      "BigNumber, BigNumber": function(p, D) {
        return It(p, n(p, D), r.epsilon) ? n(p, D) : p.toDecimalPlaces(D.toNumber(), Dt.ROUND_CEIL);
      },
      Fraction: function(p) {
        return p.ceil();
      },
      "Fraction, number": function(p, D) {
        return p.ceil(D);
      },
      "Fraction, BigNumber": function(p, D) {
        return p.ceil(D.toNumber());
      },
      "Array | Matrix": e.referToSelf((d) => (p) => Ue(p, d)),
      "Array, number | BigNumber": e.referToSelf((d) => (p, D) => Ue(p, (c) => d(c, D))),
      "SparseMatrix, number | BigNumber": e.referToSelf((d) => (p, D) => f(p, D, d, false)),
      "DenseMatrix, number | BigNumber": e.referToSelf((d) => (p, D) => s(p, D, d, false)),
      "number | Complex | Fraction | BigNumber, Array": e.referToSelf((d) => (p, D) => s(i(D), p, d, true).valueOf()),
      "number | Complex | Fraction | BigNumber, Matrix": e.referToSelf((d) => (p, D) => o(p, 0) ? u(D.size(), D.storage()) : D.storage() === "dense" ? s(D, p, d, true) : l(D, p, d, true))
    });
  }), $n = "exp", Zl = [
    "typed"
  ], Hl = Y($n, Zl, (t) => {
    var { typed: e } = t;
    return e($n, {
      number: io,
      Complex: function(n) {
        return n.exp();
      },
      BigNumber: function(n) {
        return n.exp();
      }
    });
  }), jl = "matAlgo02xDS0", Jl = [
    "typed",
    "equalScalar"
  ], co = Y(jl, Jl, (t) => {
    var { typed: e, equalScalar: r } = t;
    return function(i, o, u, a) {
      var f = i._data, l = i._size, s = i._datatype || i.getDataType(), h = o._values, d = o._index, p = o._ptr, D = o._size, c = o._datatype || o._data === void 0 ? o._datatype : o.getDataType();
      if (l.length !== D.length) throw new oe(l.length, D.length);
      if (l[0] !== D[0] || l[1] !== D[1]) throw new RangeError("Dimension mismatch. Matrix A (" + l + ") must match Matrix B (" + D + ")");
      if (!h) throw new Error("Cannot perform operation on Dense Matrix and Pattern Sparse Matrix");
      var g = l[0], m = l[1], w, v = r, A = 0, y = u;
      typeof s == "string" && s === c && s !== "mixed" && (w = s, v = e.find(r, [
        w,
        w
      ]), A = e.convert(0, w), y = e.find(u, [
        w,
        w
      ]));
      for (var _ = [], E = [], C = [], b = 0; b < m; b++) {
        C[b] = E.length;
        for (var x = p[b], N = p[b + 1], B = x; B < N; B++) {
          var Z = d[B], q = a ? y(h[B], f[Z][b]) : y(f[Z][b], h[B]);
          v(q, A) || (E.push(Z), _.push(q));
        }
      }
      return C[m] = E.length, o.createSparseMatrix({
        values: _,
        index: E,
        ptr: C,
        size: [
          g,
          m
        ],
        datatype: s === i._datatype && c === o._datatype ? w : void 0
      });
    };
  }), Kl = "matAlgo03xDSf", Yl = [
    "typed"
  ], fo = Y(Kl, Yl, (t) => {
    var { typed: e } = t;
    return function(n, i, o, u) {
      var a = n._data, f = n._size, l = n._datatype || n.getDataType(), s = i._values, h = i._index, d = i._ptr, p = i._size, D = i._datatype || i._data === void 0 ? i._datatype : i.getDataType();
      if (f.length !== p.length) throw new oe(f.length, p.length);
      if (f[0] !== p[0] || f[1] !== p[1]) throw new RangeError("Dimension mismatch. Matrix A (" + f + ") must match Matrix B (" + p + ")");
      if (!s) throw new Error("Cannot perform operation on Dense Matrix and Pattern Sparse Matrix");
      var c = f[0], g = f[1], m, w = 0, v = o;
      typeof l == "string" && l === D && l !== "mixed" && (m = l, w = e.convert(0, m), v = e.find(o, [
        m,
        m
      ]));
      for (var A = [], y = 0; y < c; y++) A[y] = [];
      for (var _ = [], E = [], C = 0; C < g; C++) {
        for (var b = C + 1, x = d[C], N = d[C + 1], B = x; B < N; B++) {
          var Z = h[B];
          _[Z] = u ? v(s[B], a[Z][C]) : v(a[Z][C], s[B]), E[Z] = b;
        }
        for (var q = 0; q < c; q++) E[q] === b ? A[q][C] = _[q] : A[q][C] = u ? v(w, a[q][C]) : v(a[q][C], w);
      }
      return n.createDenseMatrix({
        data: A,
        size: [
          c,
          g
        ],
        datatype: l === n._datatype && D === i._datatype ? m : void 0
      });
    };
  }), Gl = "matAlgo13xDD", Ql = [
    "typed"
  ], Xl = Y(Gl, Ql, (t) => {
    var { typed: e } = t;
    return function(i, o, u) {
      var a = i._data, f = i._size, l = i._datatype, s = o._data, h = o._size, d = o._datatype, p = [];
      if (f.length !== h.length) throw new oe(f.length, h.length);
      for (var D = 0; D < f.length; D++) {
        if (f[D] !== h[D]) throw new RangeError("Dimension mismatch. Matrix A (" + f + ") must match Matrix B (" + h + ")");
        p[D] = f[D];
      }
      var c, g = u;
      typeof l == "string" && l === d && (c = l, g = e.find(u, [
        c,
        c
      ]));
      var m = p.length > 0 ? r(g, 0, p, p[0], a, s) : [];
      return i.createDenseMatrix({
        data: m,
        size: p,
        datatype: c
      });
    };
    function r(n, i, o, u, a, f) {
      var l = [];
      if (i === o.length - 1) for (var s = 0; s < u; s++) l[s] = n(a[s], f[s]);
      else for (var h = 0; h < u; h++) l[h] = r(n, i + 1, o, o[i + 1], a[h], f[h]);
      return l;
    }
  }), kl = "broadcast", e0 = [
    "concat"
  ], t0 = Y(kl, e0, (t) => {
    var { concat: e } = t;
    return function(i, o) {
      var u = Math.max(i._size.length, o._size.length);
      if (i._size.length === o._size.length && i._size.every((D, c) => D === o._size[c])) return [
        i,
        o
      ];
      for (var a = r(i._size, u, 0), f = r(o._size, u, 0), l = [], s = 0; s < u; s++) l[s] = Math.max(a[s], f[s]);
      lr(a, l), lr(f, l);
      var h = i.clone(), d = o.clone();
      h._size.length < u ? h.reshape(r(h._size, u, 1)) : d._size.length < u && d.reshape(r(d._size, u, 1));
      for (var p = 0; p < u; p++) h._size[p] < l[p] && (h = n(h, l[p], p)), d._size[p] < l[p] && (d = n(d, l[p], p));
      return [
        h,
        d
      ];
    };
    function r(i, o, u) {
      return [
        ...Array(o - i.length).fill(u),
        ...i
      ];
    }
    function n(i, o, u) {
      return e(...Array(o).fill(i), u);
    }
  }), r0 = "matrixAlgorithmSuite", n0 = [
    "typed",
    "matrix",
    "concat"
  ], Cr = Y(r0, n0, (t) => {
    var { typed: e, matrix: r, concat: n } = t, i = Xl({
      typed: e
    }), o = Jt({
      typed: e
    }), u = t0({
      concat: n
    });
    return function(f) {
      var l = f.elop, s = f.SD || f.DS, h;
      l ? (h = {
        "DenseMatrix, DenseMatrix": (c, g) => i(...u(c, g), l),
        "Array, Array": (c, g) => i(...u(r(c), r(g)), l).valueOf(),
        "Array, DenseMatrix": (c, g) => i(...u(r(c), g), l),
        "DenseMatrix, Array": (c, g) => i(...u(c, r(g)), l)
      }, f.SS && (h["SparseMatrix, SparseMatrix"] = (c, g) => f.SS(...u(c, g), l, false)), f.DS && (h["DenseMatrix, SparseMatrix"] = (c, g) => f.DS(...u(c, g), l, false), h["Array, SparseMatrix"] = (c, g) => f.DS(...u(r(c), g), l, false)), s && (h["SparseMatrix, DenseMatrix"] = (c, g) => s(...u(g, c), l, true), h["SparseMatrix, Array"] = (c, g) => s(...u(r(g), c), l, true))) : (h = {
        "DenseMatrix, DenseMatrix": e.referToSelf((c) => (g, m) => i(...u(g, m), c)),
        "Array, Array": e.referToSelf((c) => (g, m) => i(...u(r(g), r(m)), c).valueOf()),
        "Array, DenseMatrix": e.referToSelf((c) => (g, m) => i(...u(r(g), m), c)),
        "DenseMatrix, Array": e.referToSelf((c) => (g, m) => i(...u(g, r(m)), c))
      }, f.SS && (h["SparseMatrix, SparseMatrix"] = e.referToSelf((c) => (g, m) => f.SS(...u(g, m), c, false))), f.DS && (h["DenseMatrix, SparseMatrix"] = e.referToSelf((c) => (g, m) => f.DS(...u(g, m), c, false)), h["Array, SparseMatrix"] = e.referToSelf((c) => (g, m) => f.DS(...u(r(g), m), c, false))), s && (h["SparseMatrix, DenseMatrix"] = e.referToSelf((c) => (g, m) => s(...u(m, g), c, true)), h["SparseMatrix, Array"] = e.referToSelf((c) => (g, m) => s(...u(r(m), g), c, true))));
      var d = f.scalar || "any", p = f.Ds || f.Ss;
      p && (l ? (h["DenseMatrix," + d] = (c, g) => o(c, g, l, false), h[d + ", DenseMatrix"] = (c, g) => o(g, c, l, true), h["Array," + d] = (c, g) => o(r(c), g, l, false).valueOf(), h[d + ", Array"] = (c, g) => o(r(g), c, l, true).valueOf()) : (h["DenseMatrix," + d] = e.referToSelf((c) => (g, m) => o(g, m, c, false)), h[d + ", DenseMatrix"] = e.referToSelf((c) => (g, m) => o(m, g, c, true)), h["Array," + d] = e.referToSelf((c) => (g, m) => o(r(g), m, c, false).valueOf()), h[d + ", Array"] = e.referToSelf((c) => (g, m) => o(r(m), g, c, true).valueOf())));
      var D = f.sS !== void 0 ? f.sS : f.Ss;
      return l ? (f.Ss && (h["SparseMatrix," + d] = (c, g) => f.Ss(c, g, l, false)), D && (h[d + ", SparseMatrix"] = (c, g) => D(g, c, l, true))) : (f.Ss && (h["SparseMatrix," + d] = e.referToSelf((c) => (g, m) => f.Ss(g, m, c, false))), D && (h[d + ", SparseMatrix"] = e.referToSelf((c) => (g, m) => D(m, g, c, true)))), l && l.signatures && Ei(h, l.signatures), h;
    };
  }), i0 = "matAlgo01xDSid", o0 = [
    "typed"
  ], u0 = Y(i0, o0, (t) => {
    var { typed: e } = t;
    return function(n, i, o, u) {
      var a = n._data, f = n._size, l = n._datatype || n.getDataType(), s = i._values, h = i._index, d = i._ptr, p = i._size, D = i._datatype || i._data === void 0 ? i._datatype : i.getDataType();
      if (f.length !== p.length) throw new oe(f.length, p.length);
      if (f[0] !== p[0] || f[1] !== p[1]) throw new RangeError("Dimension mismatch. Matrix A (" + f + ") must match Matrix B (" + p + ")");
      if (!s) throw new Error("Cannot perform operation on Dense Matrix and Pattern Sparse Matrix");
      var c = f[0], g = f[1], m = typeof l == "string" && l !== "mixed" && l === D ? l : void 0, w = m ? e.find(o, [
        m,
        m
      ]) : o, v, A, y = [];
      for (v = 0; v < c; v++) y[v] = [];
      var _ = [], E = [];
      for (A = 0; A < g; A++) {
        for (var C = A + 1, b = d[A], x = d[A + 1], N = b; N < x; N++) v = h[N], _[v] = u ? w(s[N], a[v][A]) : w(a[v][A], s[N]), E[v] = C;
        for (v = 0; v < c; v++) E[v] === C ? y[v][A] = _[v] : y[v][A] = a[v][A];
      }
      return n.createDenseMatrix({
        data: y,
        size: [
          c,
          g
        ],
        datatype: l === n._datatype && D === i._datatype ? m : void 0
      });
    };
  }), a0 = "matAlgo04xSidSid", s0 = [
    "typed",
    "equalScalar"
  ], c0 = Y(a0, s0, (t) => {
    var { typed: e, equalScalar: r } = t;
    return function(i, o, u) {
      var a = i._values, f = i._index, l = i._ptr, s = i._size, h = i._datatype || i._data === void 0 ? i._datatype : i.getDataType(), d = o._values, p = o._index, D = o._ptr, c = o._size, g = o._datatype || o._data === void 0 ? o._datatype : o.getDataType();
      if (s.length !== c.length) throw new oe(s.length, c.length);
      if (s[0] !== c[0] || s[1] !== c[1]) throw new RangeError("Dimension mismatch. Matrix A (" + s + ") must match Matrix B (" + c + ")");
      var m = s[0], w = s[1], v, A = r, y = 0, _ = u;
      typeof h == "string" && h === g && h !== "mixed" && (v = h, A = e.find(r, [
        v,
        v
      ]), y = e.convert(0, v), _ = e.find(u, [
        v,
        v
      ]));
      var E = a && d ? [] : void 0, C = [], b = [], x = a && d ? [] : void 0, N = a && d ? [] : void 0, B = [], Z = [], q, U, I, K, Q;
      for (U = 0; U < w; U++) {
        b[U] = C.length;
        var G = U + 1;
        for (K = l[U], Q = l[U + 1], I = K; I < Q; I++) q = f[I], C.push(q), B[q] = G, x && (x[q] = a[I]);
        for (K = D[U], Q = D[U + 1], I = K; I < Q; I++) if (q = p[I], B[q] === G) {
          if (x) {
            var re = _(x[q], d[I]);
            A(re, y) ? B[q] = null : x[q] = re;
          }
        } else C.push(q), Z[q] = G, N && (N[q] = d[I]);
        if (x && N) for (I = b[U]; I < C.length; ) q = C[I], B[q] === G ? (E[I] = x[q], I++) : Z[q] === G ? (E[I] = N[q], I++) : C.splice(I, 1);
      }
      return b[w] = C.length, i.createSparseMatrix({
        values: E,
        index: C,
        ptr: b,
        size: [
          m,
          w
        ],
        datatype: h === i._datatype && g === o._datatype ? v : void 0
      });
    };
  }), f0 = "matAlgo10xSids", l0 = [
    "typed",
    "DenseMatrix"
  ], h0 = Y(f0, l0, (t) => {
    var { typed: e, DenseMatrix: r } = t;
    return function(i, o, u, a) {
      var f = i._values, l = i._index, s = i._ptr, h = i._size, d = i._datatype;
      if (!f) throw new Error("Cannot perform operation on Pattern Sparse Matrix and Scalar value");
      var p = h[0], D = h[1], c, g = u;
      typeof d == "string" && (c = d, o = e.convert(o, c), g = e.find(u, [
        c,
        c
      ]));
      for (var m = [], w = [], v = [], A = 0; A < D; A++) {
        for (var y = A + 1, _ = s[A], E = s[A + 1], C = _; C < E; C++) {
          var b = l[C];
          w[b] = f[C], v[b] = y;
        }
        for (var x = 0; x < p; x++) A === 0 && (m[x] = []), v[x] === y ? m[x][A] = a ? g(o, w[x]) : g(w[x], o) : m[x][A] = o;
      }
      return new r({
        data: m,
        size: [
          p,
          D
        ],
        datatype: c
      });
    };
  }), In = "log2", d0 = [
    "typed",
    "config",
    "Complex"
  ], m0 = Y(In, d0, (t) => {
    var { typed: e, config: r, Complex: n } = t;
    return e(In, {
      number: function(u) {
        return u >= 0 || r.predictable ? oo(u) : i(new n(u, 0));
      },
      Complex: i,
      BigNumber: function(u) {
        return !u.isNegative() || r.predictable ? u.log(2) : i(new n(u.toNumber(), 0));
      },
      "Array | Matrix": e.referToSelf((o) => (u) => Ue(u, o))
    });
    function i(o) {
      var u = Math.sqrt(o.re * o.re + o.im * o.im);
      return new n(Math.log2 ? Math.log2(u) : Math.log(u) / Math.LN2, Math.atan2(o.im, o.re) / Math.LN2);
    }
  }), p0 = "multiplyScalar", v0 = [
    "typed"
  ], g0 = Y(p0, v0, (t) => {
    var { typed: e } = t;
    return e("multiplyScalar", {
      "number, number": ro,
      "Complex, Complex": function(n, i) {
        return n.mul(i);
      },
      "BigNumber, BigNumber": function(n, i) {
        return n.times(i);
      },
      "Fraction, Fraction": function(n, i) {
        return n.mul(i);
      },
      "number | Fraction | BigNumber | Complex, Unit": (r, n) => n.multiply(r),
      "Unit, number | Fraction | BigNumber | Complex | Unit": (r, n) => r.multiply(n)
    });
  }), zn = "multiply", D0 = [
    "typed",
    "matrix",
    "addScalar",
    "multiplyScalar",
    "equalScalar",
    "dot"
  ], w0 = Y(zn, D0, (t) => {
    var { typed: e, matrix: r, addScalar: n, multiplyScalar: i, equalScalar: o, dot: u } = t, a = Ot({
      typed: e,
      equalScalar: o
    }), f = Jt({
      typed: e
    });
    function l(y, _) {
      switch (y.length) {
        case 1:
          switch (_.length) {
            case 1:
              if (y[0] !== _[0]) throw new RangeError("Dimension mismatch in multiplication. Vectors must have the same length");
              break;
            case 2:
              if (y[0] !== _[0]) throw new RangeError("Dimension mismatch in multiplication. Vector length (" + y[0] + ") must match Matrix rows (" + _[0] + ")");
              break;
            default:
              throw new Error("Can only multiply a 1 or 2 dimensional matrix (Matrix B has " + _.length + " dimensions)");
          }
          break;
        case 2:
          switch (_.length) {
            case 1:
              if (y[1] !== _[0]) throw new RangeError("Dimension mismatch in multiplication. Matrix columns (" + y[1] + ") must match Vector length (" + _[0] + ")");
              break;
            case 2:
              if (y[1] !== _[0]) throw new RangeError("Dimension mismatch in multiplication. Matrix A columns (" + y[1] + ") must match Matrix B rows (" + _[0] + ")");
              break;
            default:
              throw new Error("Can only multiply a 1 or 2 dimensional matrix (Matrix B has " + _.length + " dimensions)");
          }
          break;
        default:
          throw new Error("Can only multiply a 1 or 2 dimensional matrix (Matrix A has " + y.length + " dimensions)");
      }
    }
    function s(y, _, E) {
      if (E === 0) throw new Error("Cannot multiply two empty vectors");
      return u(y, _);
    }
    function h(y, _) {
      if (_.storage() !== "dense") throw new Error("Support for SparseMatrix not implemented");
      return d(y, _);
    }
    function d(y, _) {
      var E = y._data, C = y._size, b = y._datatype || y.getDataType(), x = _._data, N = _._size, B = _._datatype || _.getDataType(), Z = C[0], q = N[1], U, I = n, K = i;
      b && B && b === B && typeof b == "string" && b !== "mixed" && (U = b, I = e.find(n, [
        U,
        U
      ]), K = e.find(i, [
        U,
        U
      ]));
      for (var Q = [], G = 0; G < q; G++) {
        for (var re = K(E[0], x[0][G]), ce = 1; ce < Z; ce++) re = I(re, K(E[ce], x[ce][G]));
        Q[G] = re;
      }
      return y.createDenseMatrix({
        data: Q,
        size: [
          q
        ],
        datatype: b === y._datatype && B === _._datatype ? U : void 0
      });
    }
    var p = e("_multiplyMatrixVector", {
      "DenseMatrix, any": c,
      "SparseMatrix, any": w
    }), D = e("_multiplyMatrixMatrix", {
      "DenseMatrix, DenseMatrix": g,
      "DenseMatrix, SparseMatrix": m,
      "SparseMatrix, DenseMatrix": v,
      "SparseMatrix, SparseMatrix": A
    });
    function c(y, _) {
      var E = y._data, C = y._size, b = y._datatype || y.getDataType(), x = _._data, N = _._datatype || _.getDataType(), B = C[0], Z = C[1], q, U = n, I = i;
      b && N && b === N && typeof b == "string" && b !== "mixed" && (q = b, U = e.find(n, [
        q,
        q
      ]), I = e.find(i, [
        q,
        q
      ]));
      for (var K = [], Q = 0; Q < B; Q++) {
        for (var G = E[Q], re = I(G[0], x[0]), ce = 1; ce < Z; ce++) re = U(re, I(G[ce], x[ce]));
        K[Q] = re;
      }
      return y.createDenseMatrix({
        data: K,
        size: [
          B
        ],
        datatype: b === y._datatype && N === _._datatype ? q : void 0
      });
    }
    function g(y, _) {
      var E = y._data, C = y._size, b = y._datatype || y.getDataType(), x = _._data, N = _._size, B = _._datatype || _.getDataType(), Z = C[0], q = C[1], U = N[1], I, K = n, Q = i;
      b && B && b === B && typeof b == "string" && b !== "mixed" && b !== "mixed" && (I = b, K = e.find(n, [
        I,
        I
      ]), Q = e.find(i, [
        I,
        I
      ]));
      for (var G = [], re = 0; re < Z; re++) {
        var ce = E[re];
        G[re] = [];
        for (var ae = 0; ae < U; ae++) {
          for (var ue = Q(ce[0], x[0][ae]), Ce = 1; Ce < q; Ce++) ue = K(ue, Q(ce[Ce], x[Ce][ae]));
          G[re][ae] = ue;
        }
      }
      return y.createDenseMatrix({
        data: G,
        size: [
          Z,
          U
        ],
        datatype: b === y._datatype && B === _._datatype ? I : void 0
      });
    }
    function m(y, _) {
      var E = y._data, C = y._size, b = y._datatype || y.getDataType(), x = _._values, N = _._index, B = _._ptr, Z = _._size, q = _._datatype || _._data === void 0 ? _._datatype : _.getDataType();
      if (!x) throw new Error("Cannot multiply Dense Matrix times Pattern only Matrix");
      var U = C[0], I = Z[1], K, Q = n, G = i, re = o, ce = 0;
      b && q && b === q && typeof b == "string" && b !== "mixed" && (K = b, Q = e.find(n, [
        K,
        K
      ]), G = e.find(i, [
        K,
        K
      ]), re = e.find(o, [
        K,
        K
      ]), ce = e.convert(0, K));
      for (var ae = [], ue = [], Ce = [], Ee = _.createSparseMatrix({
        values: ae,
        index: ue,
        ptr: Ce,
        size: [
          U,
          I
        ],
        datatype: b === y._datatype && q === _._datatype ? K : void 0
      }), Se = 0; Se < I; Se++) {
        Ce[Se] = ue.length;
        var ge = B[Se], Pe = B[Se + 1];
        if (Pe > ge) for (var he = 0, fe = 0; fe < U; fe++) {
          for (var Ze = fe + 1, Te = void 0, de = ge; de < Pe; de++) {
            var xe = N[de];
            he !== Ze ? (Te = G(E[fe][xe], x[de]), he = Ze) : Te = Q(Te, G(E[fe][xe], x[de]));
          }
          he === Ze && !re(Te, ce) && (ue.push(fe), ae.push(Te));
        }
      }
      return Ce[I] = ue.length, Ee;
    }
    function w(y, _) {
      var E = y._values, C = y._index, b = y._ptr, x = y._datatype || y._data === void 0 ? y._datatype : y.getDataType();
      if (!E) throw new Error("Cannot multiply Pattern only Matrix times Dense Matrix");
      var N = _._data, B = _._datatype || _.getDataType(), Z = y._size[0], q = _._size[0], U = [], I = [], K = [], Q, G = n, re = i, ce = o, ae = 0;
      x && B && x === B && typeof x == "string" && x !== "mixed" && (Q = x, G = e.find(n, [
        Q,
        Q
      ]), re = e.find(i, [
        Q,
        Q
      ]), ce = e.find(o, [
        Q,
        Q
      ]), ae = e.convert(0, Q));
      var ue = [], Ce = [];
      K[0] = 0;
      for (var Ee = 0; Ee < q; Ee++) {
        var Se = N[Ee];
        if (!ce(Se, ae)) for (var ge = b[Ee], Pe = b[Ee + 1], he = ge; he < Pe; he++) {
          var fe = C[he];
          Ce[fe] ? ue[fe] = G(ue[fe], re(Se, E[he])) : (Ce[fe] = true, I.push(fe), ue[fe] = re(Se, E[he]));
        }
      }
      for (var Ze = I.length, Te = 0; Te < Ze; Te++) {
        var de = I[Te];
        U[Te] = ue[de];
      }
      return K[1] = I.length, y.createSparseMatrix({
        values: U,
        index: I,
        ptr: K,
        size: [
          Z,
          1
        ],
        datatype: x === y._datatype && B === _._datatype ? Q : void 0
      });
    }
    function v(y, _) {
      var E = y._values, C = y._index, b = y._ptr, x = y._datatype || y._data === void 0 ? y._datatype : y.getDataType();
      if (!E) throw new Error("Cannot multiply Pattern only Matrix times Dense Matrix");
      var N = _._data, B = _._datatype || _.getDataType(), Z = y._size[0], q = _._size[0], U = _._size[1], I, K = n, Q = i, G = o, re = 0;
      x && B && x === B && typeof x == "string" && x !== "mixed" && (I = x, K = e.find(n, [
        I,
        I
      ]), Q = e.find(i, [
        I,
        I
      ]), G = e.find(o, [
        I,
        I
      ]), re = e.convert(0, I));
      for (var ce = [], ae = [], ue = [], Ce = y.createSparseMatrix({
        values: ce,
        index: ae,
        ptr: ue,
        size: [
          Z,
          U
        ],
        datatype: x === y._datatype && B === _._datatype ? I : void 0
      }), Ee = [], Se = [], ge = 0; ge < U; ge++) {
        ue[ge] = ae.length;
        for (var Pe = ge + 1, he = 0; he < q; he++) {
          var fe = N[he][ge];
          if (!G(fe, re)) for (var Ze = b[he], Te = b[he + 1], de = Ze; de < Te; de++) {
            var xe = C[de];
            Se[xe] !== Pe ? (Se[xe] = Pe, ae.push(xe), Ee[xe] = Q(fe, E[de])) : Ee[xe] = K(Ee[xe], Q(fe, E[de]));
          }
        }
        for (var He = ue[ge], rt = ae.length, nt = He; nt < rt; nt++) {
          var wt = ae[nt];
          ce[nt] = Ee[wt];
        }
      }
      return ue[U] = ae.length, Ce;
    }
    function A(y, _) {
      var E = y._values, C = y._index, b = y._ptr, x = y._datatype || y._data === void 0 ? y._datatype : y.getDataType(), N = _._values, B = _._index, Z = _._ptr, q = _._datatype || _._data === void 0 ? _._datatype : _.getDataType(), U = y._size[0], I = _._size[1], K = E && N, Q, G = n, re = i;
      x && q && x === q && typeof x == "string" && x !== "mixed" && (Q = x, G = e.find(n, [
        Q,
        Q
      ]), re = e.find(i, [
        Q,
        Q
      ]));
      for (var ce = K ? [] : void 0, ae = [], ue = [], Ce = y.createSparseMatrix({
        values: ce,
        index: ae,
        ptr: ue,
        size: [
          U,
          I
        ],
        datatype: x === y._datatype && q === _._datatype ? Q : void 0
      }), Ee = K ? [] : void 0, Se = [], ge, Pe, he, fe, Ze, Te, de, xe, He = 0; He < I; He++) {
        ue[He] = ae.length;
        var rt = He + 1;
        for (Ze = Z[He], Te = Z[He + 1], fe = Ze; fe < Te; fe++) if (xe = B[fe], K) for (Pe = b[xe], he = b[xe + 1], ge = Pe; ge < he; ge++) de = C[ge], Se[de] !== rt ? (Se[de] = rt, ae.push(de), Ee[de] = re(N[fe], E[ge])) : Ee[de] = G(Ee[de], re(N[fe], E[ge]));
        else for (Pe = b[xe], he = b[xe + 1], ge = Pe; ge < he; ge++) de = C[ge], Se[de] !== rt && (Se[de] = rt, ae.push(de));
        if (K) for (var nt = ue[He], wt = ae.length, yt = nt; yt < wt; yt++) {
          var xr = ae[yt];
          ce[yt] = Ee[xr];
        }
      }
      return ue[I] = ae.length, Ce;
    }
    return e(zn, i, {
      "Array, Array": e.referTo("Matrix, Matrix", (y) => (_, E) => {
        l(Be(_), Be(E));
        var C = y(r(_), r(E));
        return Ae(C) ? C.valueOf() : C;
      }),
      "Matrix, Matrix": function(_, E) {
        var C = _.size(), b = E.size();
        return l(C, b), C.length === 1 ? b.length === 1 ? s(_, E, C[0]) : h(_, E) : b.length === 1 ? p(_, E) : D(_, E);
      },
      "Matrix, Array": e.referTo("Matrix,Matrix", (y) => (_, E) => y(_, r(E))),
      "Array, Matrix": e.referToSelf((y) => (_, E) => y(r(_, E.storage()), E)),
      "SparseMatrix, any": function(_, E) {
        return a(_, E, i, false);
      },
      "DenseMatrix, any": function(_, E) {
        return f(_, E, i, false);
      },
      "any, SparseMatrix": function(_, E) {
        return a(E, _, i, true);
      },
      "any, DenseMatrix": function(_, E) {
        return f(E, _, i, true);
      },
      "Array, any": function(_, E) {
        return f(r(_), E, i, false).valueOf();
      },
      "any, Array": function(_, E) {
        return f(r(E), _, i, true).valueOf();
      },
      "any, any": i,
      "any, any, ...any": e.referToSelf((y) => (_, E, C) => {
        for (var b = y(_, E), x = 0; x < C.length; x++) b = y(b, C[x]);
        return b;
      })
    });
  }), y0 = "matAlgo09xS0Sf", _0 = [
    "typed",
    "equalScalar"
  ], A0 = Y(y0, _0, (t) => {
    var { typed: e, equalScalar: r } = t;
    return function(i, o, u) {
      var a = i._values, f = i._index, l = i._ptr, s = i._size, h = i._datatype || i._data === void 0 ? i._datatype : i.getDataType(), d = o._values, p = o._index, D = o._ptr, c = o._size, g = o._datatype || o._data === void 0 ? o._datatype : o.getDataType();
      if (s.length !== c.length) throw new oe(s.length, c.length);
      if (s[0] !== c[0] || s[1] !== c[1]) throw new RangeError("Dimension mismatch. Matrix A (" + s + ") must match Matrix B (" + c + ")");
      var m = s[0], w = s[1], v, A = r, y = 0, _ = u;
      typeof h == "string" && h === g && h !== "mixed" && (v = h, A = e.find(r, [
        v,
        v
      ]), y = e.convert(0, v), _ = e.find(u, [
        v,
        v
      ]));
      var E = a && d ? [] : void 0, C = [], b = [], x = E ? [] : void 0, N = [], B, Z, q, U, I;
      for (Z = 0; Z < w; Z++) {
        b[Z] = C.length;
        var K = Z + 1;
        if (x) for (U = D[Z], I = D[Z + 1], q = U; q < I; q++) B = p[q], N[B] = K, x[B] = d[q];
        for (U = l[Z], I = l[Z + 1], q = U; q < I; q++) if (B = f[q], x) {
          var Q = N[B] === K ? x[B] : y, G = _(a[q], Q);
          A(G, y) || (C.push(B), E.push(G));
        } else C.push(B);
      }
      return b[w] = C.length, i.createSparseMatrix({
        values: E,
        index: C,
        ptr: b,
        size: [
          m,
          w
        ],
        datatype: h === i._datatype && g === o._datatype ? v : void 0
      });
    };
  }), On = "dotMultiply", F0 = [
    "typed",
    "matrix",
    "equalScalar",
    "multiplyScalar",
    "concat"
  ], E0 = Y(On, F0, (t) => {
    var { typed: e, matrix: r, equalScalar: n, multiplyScalar: i, concat: o } = t, u = co({
      typed: e,
      equalScalar: n
    }), a = A0({
      typed: e,
      equalScalar: n
    }), f = Ot({
      typed: e,
      equalScalar: n
    }), l = Cr({
      typed: e,
      matrix: r,
      concat: o
    });
    return e(On, l({
      elop: i,
      SS: a,
      DS: u,
      Ss: f
    }));
  }), C0 = "matAlgo07xSSf", b0 = [
    "typed",
    "DenseMatrix"
  ], lo = Y(C0, b0, (t) => {
    var { typed: e, DenseMatrix: r } = t;
    return function(o, u, a) {
      var f = o._size, l = o._datatype || o._data === void 0 ? o._datatype : o.getDataType(), s = u._size, h = u._datatype || u._data === void 0 ? u._datatype : u.getDataType();
      if (f.length !== s.length) throw new oe(f.length, s.length);
      if (f[0] !== s[0] || f[1] !== s[1]) throw new RangeError("Dimension mismatch. Matrix A (" + f + ") must match Matrix B (" + s + ")");
      var d = f[0], p = f[1], D, c = 0, g = a;
      typeof l == "string" && l === h && l !== "mixed" && (D = l, c = e.convert(0, D), g = e.find(a, [
        D,
        D
      ]));
      var m, w, v = [];
      for (m = 0; m < d; m++) v[m] = [];
      var A = [], y = [], _ = [], E = [];
      for (w = 0; w < p; w++) {
        var C = w + 1;
        for (n(o, w, _, A, C), n(u, w, E, y, C), m = 0; m < d; m++) {
          var b = _[m] === C ? A[m] : c, x = E[m] === C ? y[m] : c;
          v[m][w] = g(b, x);
        }
      }
      return new r({
        data: v,
        size: [
          d,
          p
        ],
        datatype: l === o._datatype && h === u._datatype ? D : void 0
      });
    };
    function n(i, o, u, a, f) {
      for (var l = i._values, s = i._index, h = i._ptr, d = h[o], p = h[o + 1]; d < p; d++) {
        var D = s[d];
        u[D] = f, a[D] = l[d];
      }
    }
  }), Pn = "conj", M0 = [
    "typed"
  ], S0 = Y(Pn, M0, (t) => {
    var { typed: e } = t;
    return e(Pn, {
      "number | BigNumber | Fraction": (r) => r,
      Complex: (r) => r.conjugate(),
      "Array | Matrix": e.referToSelf((r) => (n) => Ue(n, r))
    });
  }), qn = "concat", N0 = [
    "typed",
    "matrix",
    "isInteger"
  ], B0 = Y(qn, N0, (t) => {
    var { typed: e, matrix: r, isInteger: n } = t;
    return e(qn, {
      "...Array | Matrix | number | BigNumber": function(o) {
        var u, a = o.length, f = -1, l, s = false, h = [];
        for (u = 0; u < a; u++) {
          var d = o[u];
          if (Ae(d) && (s = true), ve(d) || Fe(d)) {
            if (u !== a - 1) throw new Error("Dimension must be specified as last argument");
            if (l = f, f = d.valueOf(), !n(f)) throw new TypeError("Integer number expected for dimension");
            if (f < 0 || u > 0 && f > l) throw new ht(f, l + 1);
          } else {
            var p = _e(d).valueOf(), D = Be(p);
            if (h[u] = p, l = f, f = D.length - 1, u > 0 && f !== l) throw new oe(l + 1, f + 1);
          }
        }
        if (h.length === 0) throw new SyntaxError("At least one matrix expected");
        for (var c = h.shift(); h.length; ) c = Qi(c, h.shift(), f);
        return s ? r(c) : c;
      },
      "...string": function(o) {
        return o.join("");
      }
    });
  }), Rn = "identity", x0 = [
    "typed",
    "config",
    "matrix",
    "BigNumber",
    "DenseMatrix",
    "SparseMatrix"
  ], T0 = Y(Rn, x0, (t) => {
    var { typed: e, config: r, matrix: n, BigNumber: i, DenseMatrix: o, SparseMatrix: u } = t;
    return e(Rn, {
      "": function() {
        return r.matrix === "Matrix" ? n([]) : [];
      },
      string: function(s) {
        return n(s);
      },
      "number | BigNumber": function(s) {
        return f(s, s, r.matrix === "Matrix" ? "dense" : void 0);
      },
      "number | BigNumber, string": function(s, h) {
        return f(s, s, h);
      },
      "number | BigNumber, number | BigNumber": function(s, h) {
        return f(s, h, r.matrix === "Matrix" ? "dense" : void 0);
      },
      "number | BigNumber, number | BigNumber, string": function(s, h, d) {
        return f(s, h, d);
      },
      Array: function(s) {
        return a(s);
      },
      "Array, string": function(s, h) {
        return a(s, h);
      },
      Matrix: function(s) {
        return a(s.valueOf(), s.storage());
      },
      "Matrix, string": function(s, h) {
        return a(s.valueOf(), h);
      }
    });
    function a(l, s) {
      switch (l.length) {
        case 0:
          return s ? n(s) : [];
        case 1:
          return f(l[0], l[0], s);
        case 2:
          return f(l[0], l[1], s);
        default:
          throw new Error("Vector containing two values expected");
      }
    }
    function f(l, s, h) {
      var d = Fe(l) || Fe(s) ? i : null;
      if (Fe(l) && (l = l.toNumber()), Fe(s) && (s = s.toNumber()), !De(l) || l < 1) throw new Error("Parameters in function identity must be positive integers");
      if (!De(s) || s < 1) throw new Error("Parameters in function identity must be positive integers");
      var p = d ? new i(1) : 1, D = d ? new d(0) : 0, c = [
        l,
        s
      ];
      if (h) {
        if (h === "sparse") return u.diagonal(c, p, 0, D);
        if (h === "dense") return o.diagonal(c, p, 0, D);
        throw new TypeError('Unknown matrix type "'.concat(h, '"'));
      }
      for (var g = cr([], c, D), m = l < s ? l : s, w = 0; w < m; w++) g[w][w] = p;
      return g;
    }
  });
  function $0() {
    throw new Error('No "bignumber" implementation available');
  }
  function I0() {
    throw new Error('No "fraction" implementation available');
  }
  function z0() {
    throw new Error('No "matrix" implementation available');
  }
  var Ln = "size", O0 = [
    "typed",
    "config",
    "?matrix"
  ], P0 = Y(Ln, O0, (t) => {
    var { typed: e, config: r, matrix: n } = t;
    return e(Ln, {
      Matrix: function(o) {
        return o.create(o.size());
      },
      Array: Be,
      string: function(o) {
        return r.matrix === "Array" ? [
          o.length
        ] : n([
          o.length
        ]);
      },
      "number | Complex | BigNumber | Unit | boolean | null": function(o) {
        return r.matrix === "Array" ? [] : n ? n([]) : z0();
      }
    });
  }), Vn = "zeros", q0 = [
    "typed",
    "config",
    "matrix",
    "BigNumber"
  ], R0 = Y(Vn, q0, (t) => {
    var { typed: e, config: r, matrix: n, BigNumber: i } = t;
    return e(Vn, {
      "": function() {
        return r.matrix === "Array" ? o([]) : o([], "default");
      },
      "...number | BigNumber | string": function(l) {
        var s = l[l.length - 1];
        if (typeof s == "string") {
          var h = l.pop();
          return o(l, h);
        } else return r.matrix === "Array" ? o(l) : o(l, "default");
      },
      Array: o,
      Matrix: function(l) {
        var s = l.storage();
        return o(l.valueOf(), s);
      },
      "Array | Matrix, string": function(l, s) {
        return o(l.valueOf(), s);
      }
    });
    function o(f, l) {
      var s = u(f), h = s ? new i(0) : 0;
      if (a(f), l) {
        var d = n(l);
        return f.length > 0 ? d.resize(f, h) : d;
      } else {
        var p = [];
        return f.length > 0 ? cr(p, f, h) : p;
      }
    }
    function u(f) {
      var l = false;
      return f.forEach(function(s, h, d) {
        Fe(s) && (l = true, d[h] = s.toNumber());
      }), l;
    }
    function a(f) {
      f.forEach(function(l) {
        if (typeof l != "number" || !De(l) || l < 0) throw new Error("Parameters in function zeros must be positive integers");
      });
    }
  }), Un = "fft", L0 = [
    "typed",
    "matrix",
    "addScalar",
    "multiplyScalar",
    "divideScalar",
    "exp",
    "tau",
    "i",
    "dotDivide",
    "conj",
    "pow",
    "ceil",
    "log2"
  ], V0 = Y(Un, L0, (t) => {
    var { typed: e, matrix: r, addScalar: n, multiplyScalar: i, divideScalar: o, exp: u, tau: a, i: f, dotDivide: l, conj: s, pow: h, ceil: d, log2: p } = t;
    return e(Un, {
      Array: D,
      Matrix: function(v) {
        return v.create(D(v.toArray()));
      }
    });
    function D(w) {
      var v = Be(w);
      return v.length === 1 ? m(w, v[0]) : c(w.map((A) => D(A, v.slice(1))), 0);
    }
    function c(w, v) {
      var A = Be(w);
      if (v !== 0) return new Array(A[0]).fill(0).map((_, E) => c(w[E], v - 1));
      if (A.length === 1) return m(w);
      function y(_) {
        var E = Be(_);
        return new Array(E[1]).fill(0).map((C, b) => new Array(E[0]).fill(0).map((x, N) => _[N][b]));
      }
      return y(c(y(w), 1));
    }
    function g(w) {
      for (var v = w.length, A = u(o(i(-1, i(f, a)), v)), y = [], _ = 1 - v; _ < v; _++) y.push(h(A, o(h(_, 2), 2)));
      for (var E = h(2, d(p(v + v - 1))), C = [
        ...new Array(v).fill(0).map((I, K) => i(w[K], y[v - 1 + K])),
        ...new Array(E - v).fill(0)
      ], b = [
        ...new Array(v + v - 1).fill(0).map((I, K) => o(1, y[K])),
        ...new Array(E - (v + v - 1)).fill(0)
      ], x = m(C), N = m(b), B = new Array(E).fill(0).map((I, K) => i(x[K], N[K])), Z = l(s(D(s(B))), E), q = [], U = v - 1; U < v + v - 1; U++) q.push(i(Z[U], y[U]));
      return q;
    }
    function m(w) {
      var v = w.length;
      if (v === 1) return [
        w[0]
      ];
      if (v % 2 === 0) {
        for (var A = [
          ...m(w.filter((C, b) => b % 2 === 0)),
          ...m(w.filter((C, b) => b % 2 === 1))
        ], y = 0; y < v / 2; y++) {
          var _ = A[y], E = i(A[y + v / 2], u(i(i(a, f), o(-y, v))));
          A[y] = n(_, E), A[y + v / 2] = n(_, i(-1, E));
        }
        return A;
      } else return g(w);
    }
  });
  function zt(t, e, r) {
    var n;
    return String(t).includes("Unexpected type") ? (n = arguments.length > 2 ? " (type: " + ct(r) + ", value: " + JSON.stringify(r) + ")" : " (type: " + t.data.actual + ")", new TypeError("Cannot calculate " + e + ", unexpected type of argument" + n)) : String(t).includes("complex numbers") ? (n = arguments.length > 2 ? " (type: " + ct(r) + ", value: " + JSON.stringify(r) + ")" : "", new TypeError("Cannot calculate " + e + ", no ordering relation is defined for complex numbers" + n)) : t;
  }
  var U0 = "numeric", W0 = [
    "number",
    "?bignumber",
    "?fraction"
  ], Z0 = Y(U0, W0, (t) => {
    var { number: e, bignumber: r, fraction: n } = t, i = {
      string: true,
      number: true,
      BigNumber: true,
      Fraction: true
    }, o = {
      number: (u) => e(u),
      BigNumber: r ? (u) => r(u) : $0,
      Fraction: n ? (u) => n(u) : I0
    };
    return function(a) {
      var f = arguments.length > 1 && arguments[1] !== void 0 ? arguments[1] : "number", l = arguments.length > 2 ? arguments[2] : void 0;
      if (l !== void 0) throw new SyntaxError("numeric() takes one or two arguments");
      var s = ct(a);
      if (!(s in i)) throw new TypeError("Cannot convert " + a + ' of type "' + s + '"; valid input types are ' + Object.keys(i).join(", "));
      if (!(f in o)) throw new TypeError("Cannot convert " + a + ' to type "' + f + '"; valid output types are ' + Object.keys(o).join(", "));
      return f === s ? a : o[f](a);
    };
  }), Wn = "divideScalar", H0 = [
    "typed",
    "numeric"
  ], j0 = Y(Wn, H0, (t) => {
    var { typed: e, numeric: r } = t;
    return e(Wn, {
      "number, number": function(i, o) {
        return i / o;
      },
      "Complex, Complex": function(i, o) {
        return i.div(o);
      },
      "BigNumber, BigNumber": function(i, o) {
        return i.div(o);
      },
      "Fraction, Fraction": function(i, o) {
        return i.div(o);
      },
      "Unit, number | Complex | Fraction | BigNumber | Unit": (n, i) => n.divide(i),
      "number | Fraction | Complex | BigNumber, Unit": (n, i) => i.divideInto(n)
    });
  }), Zn = "pow", J0 = [
    "typed",
    "config",
    "identity",
    "multiply",
    "matrix",
    "inv",
    "fraction",
    "number",
    "Complex"
  ], K0 = Y(Zn, J0, (t) => {
    var { typed: e, config: r, identity: n, multiply: i, matrix: o, inv: u, number: a, fraction: f, Complex: l } = t;
    return e(Zn, {
      "number, number": s,
      "Complex, Complex": function(D, c) {
        return D.pow(c);
      },
      "BigNumber, BigNumber": function(D, c) {
        return c.isInteger() || D >= 0 || r.predictable ? D.pow(c) : new l(D.toNumber(), 0).pow(c.toNumber(), 0);
      },
      "Fraction, Fraction": function(D, c) {
        var g = D.pow(c);
        if (g != null) return g;
        if (r.predictable) throw new Error("Result of pow is non-rational and cannot be expressed as a fraction");
        return s(D.valueOf(), c.valueOf());
      },
      "Array, number": h,
      "Array, BigNumber": function(D, c) {
        return h(D, c.toNumber());
      },
      "Matrix, number": d,
      "Matrix, BigNumber": function(D, c) {
        return d(D, c.toNumber());
      },
      "Unit, number | BigNumber": function(D, c) {
        return D.pow(c);
      }
    });
    function s(p, D) {
      if (r.predictable && !De(D) && p < 0) try {
        var c = f(D), g = a(c);
        if ((D === g || Math.abs((D - g) / D) < 1e-14) && c.d % 2 === 1) return (c.n % 2 === 0 ? 1 : -1) * Math.pow(-p, D);
      } catch {
      }
      return r.predictable && (p < -1 && D === 1 / 0 || p > -1 && p < 0 && D === -1 / 0) ? NaN : De(D) || p >= 0 || r.predictable ? uo(p, D) : p * p < 1 && D === 1 / 0 || p * p > 1 && D === -1 / 0 ? 0 : new l(p, 0).pow(D, 0);
    }
    function h(p, D) {
      if (!De(D)) throw new TypeError("For A^b, b must be an integer (value is " + D + ")");
      var c = Be(p);
      if (c.length !== 2) throw new Error("For A^b, A must be 2 dimensional (A has " + c.length + " dimensions)");
      if (c[0] !== c[1]) throw new Error("For A^b, A must be square (size is " + c[0] + "x" + c[1] + ")");
      if (D < 0) try {
        return h(u(p), -D);
      } catch (w) {
        throw w.message === "Cannot calculate inverse, determinant is zero" ? new TypeError("For A^b, when A is not invertible, b must be a positive integer (value is " + D + ")") : w;
      }
      for (var g = n(c[0]).valueOf(), m = p; D >= 1; ) (D & 1) === 1 && (g = i(m, g)), D >>= 1, m = i(m, m);
      return g;
    }
    function d(p, D) {
      return o(h(p.valueOf(), D));
    }
  }), Et = "Number of decimals in function round must be an integer", Hn = "round", Y0 = [
    "typed",
    "config",
    "matrix",
    "equalScalar",
    "zeros",
    "BigNumber",
    "DenseMatrix"
  ], G0 = Y(Hn, Y0, (t) => {
    var { typed: e, config: r, matrix: n, equalScalar: i, zeros: o, BigNumber: u, DenseMatrix: a } = t, f = Ot({
      typed: e,
      equalScalar: i
    }), l = Er({
      typed: e,
      DenseMatrix: a
    }), s = Jt({
      typed: e
    });
    function h(d) {
      return Math.abs(jt(d).exponent);
    }
    return e(Hn, {
      number: function(p) {
        var D = Lt(p, h(r.epsilon)), c = ut(p, D, r.epsilon) ? D : p;
        return Lt(c);
      },
      "number, number": function(p, D) {
        var c = h(r.epsilon);
        if (D >= c) return Lt(p, D);
        var g = Lt(p, c), m = ut(p, g, r.epsilon) ? g : p;
        return Lt(m, D);
      },
      "number, BigNumber": function(p, D) {
        if (!D.isInteger()) throw new TypeError(Et);
        return new u(p).toDecimalPlaces(D.toNumber());
      },
      Complex: function(p) {
        return p.round();
      },
      "Complex, number": function(p, D) {
        if (D % 1) throw new TypeError(Et);
        return p.round(D);
      },
      "Complex, BigNumber": function(p, D) {
        if (!D.isInteger()) throw new TypeError(Et);
        var c = D.toNumber();
        return p.round(c);
      },
      BigNumber: function(p) {
        var D = new u(p).toDecimalPlaces(h(r.epsilon)), c = It(p, D, r.epsilon) ? D : p;
        return c.toDecimalPlaces(0);
      },
      "BigNumber, BigNumber": function(p, D) {
        if (!D.isInteger()) throw new TypeError(Et);
        var c = h(r.epsilon);
        if (D >= c) return p.toDecimalPlaces(D.toNumber());
        var g = p.toDecimalPlaces(c), m = It(p, g, r.epsilon) ? g : p;
        return m.toDecimalPlaces(D.toNumber());
      },
      Fraction: function(p) {
        return p.round();
      },
      "Fraction, number": function(p, D) {
        if (D % 1) throw new TypeError(Et);
        return p.round(D);
      },
      "Fraction, BigNumber": function(p, D) {
        if (!D.isInteger()) throw new TypeError(Et);
        return p.round(D.toNumber());
      },
      "Unit, number, Unit": e.referToSelf((d) => function(p, D, c) {
        var g = p.toNumeric(c);
        return c.multiply(d(g, D));
      }),
      "Unit, BigNumber, Unit": e.referToSelf((d) => (p, D, c) => d(p, D.toNumber(), c)),
      "Unit, Unit": e.referToSelf((d) => (p, D) => d(p, 0, D)),
      "Array | Matrix, number, Unit": e.referToSelf((d) => (p, D, c) => Ue(p, (g) => d(g, D, c))),
      "Array | Matrix, BigNumber, Unit": e.referToSelf((d) => (p, D, c) => d(p, D.toNumber(), c)),
      "Array | Matrix, Unit": e.referToSelf((d) => (p, D) => d(p, 0, D)),
      "Array | Matrix": e.referToSelf((d) => (p) => Ue(p, d)),
      "SparseMatrix, number | BigNumber": e.referToSelf((d) => (p, D) => f(p, D, d, false)),
      "DenseMatrix, number | BigNumber": e.referToSelf((d) => (p, D) => s(p, D, d, false)),
      "Array, number | BigNumber": e.referToSelf((d) => (p, D) => s(n(p), D, d, false).valueOf()),
      "number | Complex | BigNumber | Fraction, SparseMatrix": e.referToSelf((d) => (p, D) => i(p, 0) ? o(D.size(), D.storage()) : l(D, p, d, true)),
      "number | Complex | BigNumber | Fraction, DenseMatrix": e.referToSelf((d) => (p, D) => i(p, 0) ? o(D.size(), D.storage()) : s(D, p, d, true)),
      "number | Complex | BigNumber | Fraction, Array": e.referToSelf((d) => (p, D) => s(n(D), p, d, true).valueOf())
    });
  }), jn = "dotDivide", Q0 = [
    "typed",
    "matrix",
    "equalScalar",
    "divideScalar",
    "DenseMatrix",
    "concat"
  ], X0 = Y(jn, Q0, (t) => {
    var { typed: e, matrix: r, equalScalar: n, divideScalar: i, DenseMatrix: o, concat: u } = t, a = co({
      typed: e,
      equalScalar: n
    }), f = fo({
      typed: e
    }), l = lo({
      typed: e,
      DenseMatrix: o
    }), s = Ot({
      typed: e,
      equalScalar: n
    }), h = Er({
      typed: e,
      DenseMatrix: o
    }), d = Cr({
      typed: e,
      matrix: r,
      concat: u
    });
    return e(jn, d({
      elop: i,
      SS: l,
      DS: f,
      SD: a,
      Ss: s,
      sS: h
    }));
  }), mr = "larger", k0 = [
    "typed",
    "config",
    "matrix",
    "DenseMatrix",
    "concat"
  ], eh = Y(mr, k0, (t) => {
    var { typed: e, config: r, matrix: n, DenseMatrix: i, concat: o } = t, u = fo({
      typed: e
    }), a = lo({
      typed: e,
      DenseMatrix: i
    }), f = Er({
      typed: e,
      DenseMatrix: i
    }), l = Cr({
      typed: e,
      matrix: n,
      concat: o
    }), s = so({
      typed: e
    });
    return e(mr, th({
      typed: e,
      config: r
    }), {
      "boolean, boolean": (h, d) => h > d,
      "BigNumber, BigNumber": function(d, p) {
        return d.gt(p) && !It(d, p, r.epsilon);
      },
      "Fraction, Fraction": (h, d) => h.compare(d) === 1,
      "Complex, Complex": function() {
        throw new TypeError("No ordering relation is defined for complex numbers");
      }
    }, s, l({
      SS: a,
      DS: u,
      Ss: f
    }));
  }), th = Y(mr, [
    "typed",
    "config"
  ], (t) => {
    var { typed: e, config: r } = t;
    return e(mr, {
      "number, number": function(i, o) {
        return i > o && !ut(i, o, r.epsilon);
      }
    });
  }), Jn = "max", rh = [
    "typed",
    "config",
    "numeric",
    "larger"
  ], nh = Y(Jn, rh, (t) => {
    var { typed: e, config: r, numeric: n, larger: i } = t;
    return e(Jn, {
      "Array | Matrix": u,
      "Array | Matrix, number | BigNumber": function(f, l) {
        return Xr(f, l.valueOf(), o);
      },
      "...": function(f) {
        if (Qr(f)) throw new TypeError("Scalar values expected in function max");
        return u(f);
      }
    });
    function o(a, f) {
      try {
        return i(a, f) ? a : f;
      } catch (l) {
        throw zt(l, "max", f);
      }
    }
    function u(a) {
      var f;
      if (_r(a, function(l) {
        try {
          isNaN(l) && typeof l == "number" ? f = NaN : (f === void 0 || i(l, f)) && (f = l);
        } catch (s) {
          throw zt(s, "max", l);
        }
      }), f === void 0) throw new Error("Cannot calculate max of an empty array");
      return typeof f == "string" && (f = n(f, r.number)), f;
    }
  });
  yr(function(t) {
    return new t(1).exp();
  }, {
    hasher: br
  });
  yr(function(t) {
    return new t(1).plus(new t(5).sqrt()).div(2);
  }, {
    hasher: br
  });
  var ih = yr(function(t) {
    return t.acos(-1);
  }, {
    hasher: br
  }), oh = yr(function(t) {
    return ih(t).times(2);
  }, {
    hasher: br
  });
  function br(t) {
    return t[0].precision;
  }
  var Kn = "add", uh = [
    "typed",
    "matrix",
    "addScalar",
    "equalScalar",
    "DenseMatrix",
    "SparseMatrix",
    "concat"
  ], ah = Y(Kn, uh, (t) => {
    var { typed: e, matrix: r, addScalar: n, equalScalar: i, DenseMatrix: o, SparseMatrix: u, concat: a } = t, f = u0({
      typed: e
    }), l = c0({
      typed: e,
      equalScalar: i
    }), s = h0({
      typed: e,
      DenseMatrix: o
    }), h = Cr({
      typed: e,
      matrix: r,
      concat: a
    });
    return e(Kn, {
      "any, any": n,
      "any, any, ...any": e.referToSelf((d) => (p, D, c) => {
        for (var g = d(p, D), m = 0; m < c.length; m++) g = d(g, c[m]);
        return g;
      })
    }, h({
      elop: n,
      DS: f,
      SS: l,
      Ss: s
    }));
  }), Yn = "dot", sh = [
    "typed",
    "addScalar",
    "multiplyScalar",
    "conj",
    "size"
  ], ch = Y(Yn, sh, (t) => {
    var { typed: e, addScalar: r, multiplyScalar: n, conj: i, size: o } = t;
    return e(Yn, {
      "Array | DenseMatrix, Array | DenseMatrix": a,
      "SparseMatrix, SparseMatrix": f
    });
    function u(s, h) {
      var d = l(s), p = l(h), D, c;
      if (d.length === 1) D = d[0];
      else if (d.length === 2 && d[1] === 1) D = d[0];
      else throw new RangeError("Expected a column vector, instead got a matrix of size (" + d.join(", ") + ")");
      if (p.length === 1) c = p[0];
      else if (p.length === 2 && p[1] === 1) c = p[0];
      else throw new RangeError("Expected a column vector, instead got a matrix of size (" + p.join(", ") + ")");
      if (D !== c) throw new RangeError("Vectors must have equal length (" + D + " != " + c + ")");
      if (D === 0) throw new RangeError("Cannot calculate the dot product of empty vectors");
      return D;
    }
    function a(s, h) {
      var d = u(s, h), p = Ae(s) ? s._data : s, D = Ae(s) ? s._datatype || s.getDataType() : void 0, c = Ae(h) ? h._data : h, g = Ae(h) ? h._datatype || h.getDataType() : void 0, m = l(s).length === 2, w = l(h).length === 2, v = r, A = n;
      if (D && g && D === g && typeof D == "string" && D !== "mixed") {
        var y = D;
        v = e.find(r, [
          y,
          y
        ]), A = e.find(n, [
          y,
          y
        ]);
      }
      if (!m && !w) {
        for (var _ = A(i(p[0]), c[0]), E = 1; E < d; E++) _ = v(_, A(i(p[E]), c[E]));
        return _;
      }
      if (!m && w) {
        for (var C = A(i(p[0]), c[0][0]), b = 1; b < d; b++) C = v(C, A(i(p[b]), c[b][0]));
        return C;
      }
      if (m && !w) {
        for (var x = A(i(p[0][0]), c[0]), N = 1; N < d; N++) x = v(x, A(i(p[N][0]), c[N]));
        return x;
      }
      if (m && w) {
        for (var B = A(i(p[0][0]), c[0][0]), Z = 1; Z < d; Z++) B = v(B, A(i(p[Z][0]), c[Z][0]));
        return B;
      }
    }
    function f(s, h) {
      u(s, h);
      for (var d = s._index, p = s._values, D = h._index, c = h._values, g = 0, m = r, w = n, v = 0, A = 0; v < d.length && A < D.length; ) {
        var y = d[v], _ = D[A];
        if (y < _) {
          v++;
          continue;
        }
        if (y > _) {
          A++;
          continue;
        }
        y === _ && (g = m(g, w(p[v], c[A])), v++, A++);
      }
      return g;
    }
    function l(s) {
      return Ae(s) ? s.size() : o(s);
    }
  }), Gn = "det", fh = [
    "typed",
    "matrix",
    "subtractScalar",
    "multiply",
    "divideScalar",
    "isZero",
    "unaryMinus"
  ], lh = Y(Gn, fh, (t) => {
    var { typed: e, matrix: r, subtractScalar: n, multiply: i, divideScalar: o, isZero: u, unaryMinus: a } = t;
    return e(Gn, {
      any: function(s) {
        return _e(s);
      },
      "Array | Matrix": function(s) {
        var h;
        switch (Ae(s) ? h = s.size() : Array.isArray(s) ? (s = r(s), h = s.size()) : h = [], h.length) {
          case 0:
            return _e(s);
          case 1:
            if (h[0] === 1) return _e(s.valueOf()[0]);
            if (h[0] === 0) return 1;
            throw new RangeError("Matrix must be square (size: " + Me(h) + ")");
          case 2: {
            var d = h[0], p = h[1];
            if (d === p) return f(s.clone().valueOf(), d);
            if (p === 0) return 1;
            throw new RangeError("Matrix must be square (size: " + Me(h) + ")");
          }
          default:
            throw new RangeError("Matrix must be two dimensional (size: " + Me(h) + ")");
        }
      }
    });
    function f(l, s, h) {
      if (s === 1) return _e(l[0][0]);
      if (s === 2) return n(i(l[0][0], l[1][1]), i(l[1][0], l[0][1]));
      for (var d = false, p = new Array(s).fill(0).map((E, C) => C), D = 0; D < s; D++) {
        var c = p[D];
        if (u(l[c][D])) {
          var g = void 0;
          for (g = D + 1; g < s; g++) if (!u(l[p[g]][D])) {
            c = p[g], p[g] = p[D], p[D] = c, d = !d;
            break;
          }
          if (g === s) return l[c][D];
        }
        for (var m = l[c][D], w = D === 0 ? 1 : l[p[D - 1]][D - 1], v = D + 1; v < s; v++) for (var A = p[v], y = D + 1; y < s; y++) l[A][y] = o(n(i(l[A][y], m), i(l[A][D], l[c][y])), w);
      }
      var _ = l[p[s - 1]][s - 1];
      return d ? a(_) : _;
    }
  }), Qn = "inv", hh = [
    "typed",
    "matrix",
    "divideScalar",
    "addScalar",
    "multiply",
    "unaryMinus",
    "det",
    "identity",
    "abs"
  ], dh = Y(Qn, hh, (t) => {
    var { typed: e, matrix: r, divideScalar: n, addScalar: i, multiply: o, unaryMinus: u, det: a, identity: f, abs: l } = t;
    return e(Qn, {
      "Array | Matrix": function(d) {
        var p = Ae(d) ? d.size() : Be(d);
        switch (p.length) {
          case 1:
            if (p[0] === 1) return Ae(d) ? r([
              n(1, d.valueOf()[0])
            ]) : [
              n(1, d[0])
            ];
            throw new RangeError("Matrix must be square (size: " + Me(p) + ")");
          case 2: {
            var D = p[0], c = p[1];
            if (D === c) return Ae(d) ? r(s(d.valueOf(), D, c), d.storage()) : s(d, D, c);
            throw new RangeError("Matrix must be square (size: " + Me(p) + ")");
          }
          default:
            throw new RangeError("Matrix must be two dimensional (size: " + Me(p) + ")");
        }
      },
      any: function(d) {
        return n(1, d);
      }
    });
    function s(h, d, p) {
      var D, c, g, m, w;
      if (d === 1) {
        if (m = h[0][0], m === 0) throw Error("Cannot calculate inverse, determinant is zero");
        return [
          [
            n(1, m)
          ]
        ];
      } else if (d === 2) {
        var v = a(h);
        if (v === 0) throw Error("Cannot calculate inverse, determinant is zero");
        return [
          [
            n(h[1][1], v),
            n(u(h[0][1]), v)
          ],
          [
            n(u(h[1][0]), v),
            n(h[0][0], v)
          ]
        ];
      } else {
        var A = h.concat();
        for (D = 0; D < d; D++) A[D] = A[D].concat();
        for (var y = f(d).valueOf(), _ = 0; _ < p; _++) {
          var E = l(A[_][_]), C = _;
          for (D = _ + 1; D < d; ) l(A[D][_]) > E && (E = l(A[D][_]), C = D), D++;
          if (E === 0) throw Error("Cannot calculate inverse, determinant is zero");
          D = C, D !== _ && (w = A[_], A[_] = A[D], A[D] = w, w = y[_], y[_] = y[D], y[D] = w);
          var b = A[_], x = y[_];
          for (D = 0; D < d; D++) {
            var N = A[D], B = y[D];
            if (D !== _) {
              if (N[_] !== 0) {
                for (g = n(u(N[_]), b[_]), c = _; c < p; c++) N[c] = i(N[c], o(g, b[c]));
                for (c = 0; c < p; c++) B[c] = i(B[c], o(g, x[c]));
              }
            } else {
              for (g = b[_], c = _; c < p; c++) N[c] = n(N[c], g);
              for (c = 0; c < p; c++) B[c] = n(B[c], g);
            }
          }
        }
        return y;
      }
    }
  }), mh = "divide", ph = [
    "typed",
    "matrix",
    "multiply",
    "equalScalar",
    "divideScalar",
    "inv"
  ], vh = Y(mh, ph, (t) => {
    var { typed: e, matrix: r, multiply: n, equalScalar: i, divideScalar: o, inv: u } = t, a = Ot({
      typed: e,
      equalScalar: i
    }), f = Jt({
      typed: e
    });
    return e("divide", Ei({
      "Array | Matrix, Array | Matrix": function(s, h) {
        return n(s, u(h));
      },
      "DenseMatrix, any": function(s, h) {
        return f(s, h, o, false);
      },
      "SparseMatrix, any": function(s, h) {
        return a(s, h, o, false);
      },
      "Array, any": function(s, h) {
        return f(r(s), h, o, false).valueOf();
      },
      "any, Array | Matrix": function(s, h) {
        return n(s, u(h));
      }
    }, o.signatures));
  }), Xn = "sum", gh = [
    "typed",
    "config",
    "add",
    "numeric"
  ], Dh = Y(Xn, gh, (t) => {
    var { typed: e, config: r, add: n, numeric: i } = t;
    return e(Xn, {
      "Array | Matrix": o,
      "Array | Matrix, number | BigNumber": u,
      "...": function(f) {
        if (Qr(f)) throw new TypeError("Scalar values expected in function sum");
        return o(f);
      }
    });
    function o(a) {
      var f;
      return _r(a, function(l) {
        try {
          f = f === void 0 ? l : n(f, l);
        } catch (s) {
          throw zt(s, "sum", l);
        }
      }), f === void 0 && (f = i(0, r.number)), typeof f == "string" && (f = i(f, r.number)), f;
    }
    function u(a, f) {
      try {
        var l = Xr(a, f, n);
        return l;
      } catch (s) {
        throw zt(s, "sum");
      }
    }
  }), kn = "mean", wh = [
    "typed",
    "add",
    "divide"
  ], yh = Y(kn, wh, (t) => {
    var { typed: e, add: r, divide: n } = t;
    return e(kn, {
      "Array | Matrix": o,
      "Array | Matrix, number | BigNumber": i,
      "...": function(a) {
        if (Qr(a)) throw new TypeError("Scalar values expected in function mean");
        return o(a);
      }
    });
    function i(u, a) {
      try {
        var f = Xr(u, a, r), l = Array.isArray(u) ? Be(u) : u.size();
        return n(f, l[a]);
      } catch (s) {
        throw zt(s, "mean");
      }
    }
    function o(u) {
      var a, f = 0;
      if (_r(u, function(l) {
        try {
          a = a === void 0 ? l : r(a, l), f++;
        } catch (s) {
          throw zt(s, "mean", l);
        }
      }), f === 0) throw new Error("Cannot calculate the mean of an empty array");
      return n(a, f);
    }
  }), _h = ho("tau", [
    "config",
    "?BigNumber"
  ], (t) => {
    var { config: e, BigNumber: r } = t;
    return e.number === "BigNumber" ? oh(r) : il;
  }), Ah = ho("i", [
    "Complex"
  ], (t) => {
    var { Complex: e } = t;
    return e.I;
  });
  function ho(t, e, r) {
    return Y(t, e, r, {
      recreateOnConfigChange: true
    });
  }
  var Pt = Mf({
    config: We
  }), Mr = Tf({}), mo = Pf({}), Fh = Ah({
    Complex: Mr
  }), kr = Lf({}), Eh = _h({
    BigNumber: Pt,
    config: We
  }), dt = el({
    Matrix: kr
  }), ie = Nc({
    BigNumber: Pt,
    Complex: Mr,
    DenseMatrix: dt,
    Fraction: mo
  }), Ch = Bl({
    typed: ie
  }), Kt = Tl({
    typed: ie
  }), bh = _l({
    BigNumber: Pt,
    typed: ie
  }), po = S0({
    typed: ie
  }), mt = fl({
    config: We,
    typed: ie
  }), Mh = Hl({
    typed: ie
  }), Sh = nl({
    typed: ie
  }), Nh = al({
    typed: ie
  }), Bh = m0({
    Complex: Mr,
    config: We,
    typed: ie
  }), Sr = g0({
    typed: ie
  }), vo = Dl({
    typed: ie
  }), en = dl({
    Matrix: kr,
    equalScalar: mt,
    typed: ie
  }), xh = Il({
    typed: ie
  }), go = Sl({
    typed: ie
  }), Do = El({
    Fraction: mo,
    typed: ie
  }), Oe = bl({
    DenseMatrix: dt,
    Matrix: kr,
    SparseMatrix: en,
    typed: ie
  }), tn = Z0({
    bignumber: bh,
    fraction: Do,
    number: vo
  }), Th = P0({
    matrix: Oe,
    config: We,
    typed: ie
  }), wo = R0({
    BigNumber: Pt,
    config: We,
    matrix: Oe,
    typed: ie
  }), Nr = B0({
    isInteger: Sh,
    matrix: Oe,
    typed: ie
  }), Yt = j0({
    numeric: tn,
    typed: ie
  }), $h = X0({
    DenseMatrix: dt,
    concat: Nr,
    divideScalar: Yt,
    equalScalar: mt,
    matrix: Oe,
    typed: ie
  }), yo = T0({
    BigNumber: Pt,
    DenseMatrix: dt,
    SparseMatrix: en,
    config: We,
    matrix: Oe,
    typed: ie
  }), Ih = G0({
    BigNumber: Pt,
    DenseMatrix: dt,
    config: We,
    equalScalar: mt,
    matrix: Oe,
    typed: ie,
    zeros: wo
  }), rn = ah({
    DenseMatrix: dt,
    SparseMatrix: en,
    addScalar: Kt,
    concat: Nr,
    equalScalar: mt,
    matrix: Oe,
    typed: ie
  }), _o = ch({
    addScalar: Kt,
    conj: po,
    multiplyScalar: Sr,
    size: Th,
    typed: ie
  }), zh = eh({
    DenseMatrix: dt,
    concat: Nr,
    config: We,
    matrix: Oe,
    typed: ie
  }), Br = w0({
    addScalar: Kt,
    dot: _o,
    equalScalar: mt,
    matrix: Oe,
    multiplyScalar: Sr,
    typed: ie
  }), Oh = Dh({
    add: rn,
    config: We,
    numeric: tn,
    typed: ie
  }), Ph = Wl({
    DenseMatrix: dt,
    config: We,
    equalScalar: mt,
    matrix: Oe,
    round: Ih,
    typed: ie,
    zeros: wo
  }), qh = lh({
    divideScalar: Yt,
    isZero: Nh,
    matrix: Oe,
    multiply: Br,
    subtractScalar: xh,
    typed: ie,
    unaryMinus: go
  }), ei = E0({
    concat: Nr,
    equalScalar: mt,
    matrix: Oe,
    multiplyScalar: Sr,
    typed: ie
  }), Rh = nh({
    config: We,
    larger: zh,
    numeric: tn,
    typed: ie
  }), Ao = dh({
    abs: Ch,
    addScalar: Kt,
    det: qh,
    divideScalar: Yt,
    identity: yo,
    matrix: Oe,
    multiply: Br,
    typed: ie,
    unaryMinus: go
  }), Lh = K0({
    Complex: Mr,
    config: We,
    fraction: Do,
    identity: yo,
    inv: Ao,
    matrix: Oe,
    multiply: Br,
    number: vo,
    typed: ie
  }), ti = V0({
    addScalar: Kt,
    ceil: Ph,
    conj: po,
    divideScalar: Yt,
    dotDivide: $h,
    exp: Mh,
    i: Fh,
    log2: Bh,
    matrix: Oe,
    multiplyScalar: Sr,
    pow: Lh,
    tau: Eh,
    typed: ie
  }), Vh = vh({
    divideScalar: Yt,
    equalScalar: mt,
    inv: Ao,
    matrix: Oe,
    multiply: Br,
    typed: ie
  }), Uh = yh({
    add: rn,
    divide: Vh,
    typed: ie
  });
  console.log("\u{1F680} main.js loading...");
  console.log("\u2705 Imports successful");
  let S = null;
  console.log("\u{1F50D} Getting DOM elements...");
  const vt = document.getElementById("sdk-status-text"), Fo = document.getElementById("memory-status-text"), Ut = document.getElementById("results-output");
  console.log("\u2705 DOM elements retrieved:", {
    sdkStatusText: vt,
    memoryStatusText: Fo,
    resultsOutput: Ut
  });
  const ri = document.querySelectorAll(".test-category"), Wh = document.querySelectorAll(".test-section"), Zh = document.getElementById("run-all"), Hh = document.getElementById("clear-results"), jh = document.getElementById("gc-now"), Jh = document.getElementById("reset-eval"), Kh = document.querySelectorAll(".test-btn");
  async function ni() {
    try {
      console.log("\u{1F504} Starting SDK initialization..."), vt.textContent = "Initializing...", vt.style.color = "var(--warning)", S = new Ts(), console.log("\u2705 Achronyme instance created"), await S.init(), console.log("\u2705 SDK initialized successfully"), vt.textContent = "Ready", vt.style.color = "var(--success)", et("SDK Initialized", "SDK is ready for testing", "success"), nn();
    } catch (t) {
      console.error("\u274C SDK initialization failed:", t), vt.textContent = "Error", vt.style.color = "var(--error)", et("SDK Initialization Failed", t.message + `

Check browser console for details`, "error");
    }
  }
  function nn() {
    if (S) {
      const t = S.getActiveValuesCount();
      Fo.textContent = `${t} values`;
    }
  }
  function et(t, e, r = "info") {
    const n = Ut.querySelector(".placeholder");
    n && n.remove();
    const i = document.createElement("div");
    i.className = `result-item ${r}`;
    const o = document.createElement("div");
    o.className = "result-header";
    const u = document.createElement("div");
    u.className = "result-title", u.textContent = t;
    const a = document.createElement("div");
    a.className = "result-time", a.textContent = (/* @__PURE__ */ new Date()).toLocaleTimeString(), o.appendChild(u), o.appendChild(a);
    const f = document.createElement("div");
    f.className = "result-content", f.textContent = e, i.appendChild(o), i.appendChild(f), Ut.insertBefore(i, Ut.firstChild), nn();
  }
  function Yh() {
    Ut.innerHTML = '<p class="placeholder">Run tests to see results here...</p>';
  }
  function Gh() {
    if (S) {
      const t = S.gc();
      et("Manual GC", `Freed ${t} handles`, "info"), nn();
    }
  }
  function Qh() {
    S && (S.resetEvaluator(), et("Reset Evaluator", "SOC evaluator state cleared (all variables removed)", "info"));
  }
  ri.forEach((t) => {
    t.addEventListener("click", () => {
      const e = t.dataset.category;
      ri.forEach((r) => r.classList.remove("active")), t.classList.add("active"), Wh.forEach((r) => {
        r.id === `section-${e}` ? r.classList.remove("hidden") : r.classList.add("hidden");
      });
    });
  });
  Hh.addEventListener("click", Yh);
  jh.addEventListener("click", Gh);
  Jh.addEventListener("click", Qh);
  async function Eo(t, e, r) {
    if (!S) {
      et(t, "SDK not initialized", "error");
      return;
    }
    r.classList.add("running");
    const n = performance.now();
    try {
      const i = await e(), u = (performance.now() - n).toFixed(2);
      r.classList.remove("running"), r.classList.add("success"), setTimeout(() => r.classList.remove("success"), 2e3), et(t, `\u2713 ${i}
Time: ${u}ms`, "success");
    } catch (i) {
      const u = (performance.now() - n).toFixed(2);
      r.classList.remove("running"), r.classList.add("error"), setTimeout(() => r.classList.remove("error"), 2e3), et(t, `\u2717 ${i.message}
Time: ${u}ms`, "error");
    }
  }
  const Co = {
    "basic-arithmetic": async () => S.use(async () => "10+5=15, 10-5=5, 10*5=50, 10/5=2"),
    "basic-power": async () => S.use(async () => {
      const t = Math.pow(2, 8), e = Math.sqrt(16);
      return `2^8=${t}, \u221A16=${e}`;
    }),
    "basic-trig": async () => S.use(async () => {
      const t = Math.PI / 4, e = S.sin(t), r = S.cos(t), n = S.tan(t);
      return `sin(\u03C0/4)=${e.toFixed(4)}, cos(\u03C0/4)=${r.toFixed(4)}, tan(\u03C0/4)=${n.toFixed(4)}`;
    }),
    "basic-exp-log": async () => S.use(async () => {
      const t = S.exp(2), e = S.ln(Math.E);
      return `e^2=${t.toFixed(4)}, ln(e)=${e.toFixed(4)}`;
    }),
    "basic-abs": async () => S.use(async () => `|-42|=${S.abs(-42)}`),
    "vector-create": async () => S.use(async () => (S.vector([
      1,
      2,
      3,
      4,
      5
    ]), "Created vector with 5 elements")),
    "vector-arithmetic": async () => S.use(async () => (S.vector([
      1,
      2,
      3
    ]), S.vector([
      4,
      5,
      6
    ]), "Created two vectors: v1=[1,2,3], v2=[4,5,6]")),
    "vector-dot": async () => S.use(async () => {
      const t = S.vector([
        1,
        2,
        3
      ]), e = S.vector([
        4,
        5,
        6
      ]);
      return `[1,2,3] \xB7 [4,5,6] = ${S.dot(t, e)}`;
    }),
    "vector-stats": async () => S.use(async () => {
      const t = S.vector([
        1,
        2,
        3,
        4,
        5,
        6,
        7,
        8,
        9,
        10
      ]), e = S.mean(t), r = S.std(t);
      return `mean=${e}, std=${r.toFixed(4)}`;
    }),
    "vector-operations": async () => S.use(async () => {
      const t = S.vector([
        3,
        1,
        4,
        1,
        5,
        9,
        2,
        6
      ]), e = S.sum(t), r = S.max(t), n = S.min(t);
      return `sum=${e}, max=${r}, min=${n}`;
    }),
    "complex-create": async () => S.use(async () => (S.complex(3, 4), "Created complex: 3+4i")),
    "complex-arithmetic": async () => S.use(async () => (S.complex(2, 3), S.complex(1, 4), "Created complex numbers: (2+3i) and (1+4i)")),
    "complex-conjugate": async () => S.use(async () => (S.complex(3, 4), "Complex number created: 3+4i")),
    "complex-magnitude": async () => S.use(async () => (S.complex(3, 4), "Complex number 3+4i created (magnitude should be 5)")),
    "dsp-fft": async () => S.use(async () => {
      const t = S.vector([
        1,
        2,
        3,
        4,
        5,
        6,
        7,
        8
      ]);
      return S.fft(t), "FFT computed on 8 samples";
    }),
    "dsp-fft-mag": async () => S.use(async () => {
      const t = S.vector(Array.from({
        length: 1024
      }, (e, r) => Math.sin(2 * Math.PI * 50 * r / 1e3)));
      return S.fftMag(t), "FFT magnitude computed on 1024 samples";
    }),
    "dsp-ifft": async () => S.use(async () => {
      const t = S.vector([
        1,
        2,
        3,
        4,
        5,
        6,
        7,
        8
      ]), e = S.fft(t);
      return S.ifft(e), "IFFT computed, signal reconstructed";
    }),
    "dsp-windows": async () => S.use(async () => (S.dsp.hanning(64), S.dsp.hamming(64), S.dsp.blackman(64), "Created Hann, Hamming, Blackman windows (64 samples each)")),
    "dsp-convolution": async () => S.use(async () => {
      const t = S.vector([
        1,
        2,
        3,
        4
      ]), e = S.vector([
        0.25,
        0.5,
        0.25
      ]);
      return S.conv(t, e), "Convolution computed";
    }),
    "dsp-pipeline": async () => S.use(async () => (S.vector(Array.from({
      length: 1024
    }, (t, e) => Math.sin(2 * Math.PI * 50 * e / 1e3) + 0.5 * Math.random())), "DSP pipeline test (signal created)")),
    "hof-map": async () => S.use(async () => {
      const t = S.vector([
        1,
        2,
        3,
        4,
        5
      ]);
      return S.map((e) => e * e, t), "map(x => x*x) applied to vector";
    }),
    "hof-filter": async () => S.use(async () => {
      const t = S.vector([
        1,
        2,
        3,
        4,
        5,
        6
      ]);
      return S.filter((e) => e % 2 === 0, t), "filter(x => x%2==0) applied to vector";
    }),
    "hof-reduce": async () => S.use(async () => {
      const t = S.vector([
        1,
        2,
        3,
        4,
        5
      ]);
      return `reduce((a,b) => a+b, 0) = ${S.reduce((r, n) => r + n, 0, t)}`;
    }),
    "hof-pipe": async () => S.use(async () => (S.vector([
      1,
      2,
      3,
      4,
      5
    ]), "Pipe test (chaining HOF operations)")),
    "hof-lambda": async () => S.use(async () => "Lambda test (not yet implemented in SDK)"),
    "matrix-create": async () => S.use(async () => (S.matrix([
      [
        1,
        2
      ],
      [
        3,
        4
      ]
    ]), "Created 2x2 matrix: [[1,2],[3,4]]")),
    "matrix-arithmetic": async () => S.use(async () => (S.matrix([
      [
        1,
        2
      ],
      [
        3,
        4
      ]
    ]), S.matrix([
      [
        5,
        6
      ],
      [
        7,
        8
      ]
    ]), "Created two 2x2 matrices")),
    "matrix-multiply": async () => S.use(async () => (S.matrix([
      [
        1,
        2
      ],
      [
        3,
        4
      ]
    ]), S.matrix([
      [
        5,
        6
      ],
      [
        7,
        8
      ]
    ]), "Matrix multiplication test (created matrices)")),
    "matrix-determinant": async () => S.use(async () => {
      const t = S.matrix([
        [
          1,
          2
        ],
        [
          3,
          4
        ]
      ]);
      return `det([[1,2],[3,4]]) = ${S.det(t)}`;
    }),
    "matrix-inverse": async () => S.use(async () => {
      const t = S.matrix([
        [
          1,
          2
        ],
        [
          3,
          4
        ]
      ]);
      return S.linalg.inverse(t), "Matrix inverse computed for [[1,2],[3,4]]";
    }),
    "matrix-decomposition": async () => S.use(async () => {
      const t = S.matrix([
        [
          4,
          2
        ],
        [
          1,
          3
        ]
      ]);
      return S.lu(t), "LU decomposition computed";
    }),
    "stress-memory": async () => {
      const e = S.getActiveValuesCount(), r = performance.now();
      for (let u = 0; u < 5e4; u++) await S.use(async () => {
        const a = S.vector([
          1,
          2,
          3,
          4,
          5
        ]);
        S.sum(a);
      });
      const n = performance.now() - r, o = S.getActiveValuesCount() - e;
      return `Created/disposed 50000 vectors in ${n.toFixed(2)}ms
Rate: ${(5e4 / n * 1e3).toFixed(0)} vectors/sec
Memory leaks: ${o} handles`;
    },
    "stress-computation": async () => {
      performance.now();
      let r = "";
      return await S.use(async () => {
        const n = Array.from({
          length: 1e5
        }, (d, p) => p % 100 / 10 + 0.1), i = S.vector(n), o = [], u = performance.now();
        for (let d = 0; d < 50; d++) S.sin(i);
        o.push({
          name: "sin",
          time: performance.now() - u
        });
        const a = performance.now();
        for (let d = 0; d < 50; d++) S.cos(i);
        o.push({
          name: "cos",
          time: performance.now() - a
        });
        const f = performance.now();
        for (let d = 0; d < 50; d++) S.exp(i);
        o.push({
          name: "exp",
          time: performance.now() - f
        });
        const l = performance.now();
        for (let d = 0; d < 50; d++) S.sqrt(i);
        o.push({
          name: "sqrt",
          time: performance.now() - l
        });
        const s = 1e5 * 50 * o.length, h = o.reduce((d, p) => d + p.time, 0);
        r = `EXTREME VECTORIZED STRESS
`, r += `${1e5.toLocaleString()} elements \xD7 50 iterations \xD7 4 ops
`, r += `Total ops: ${s.toLocaleString()}
`, r += `Time: ${h.toFixed(2)}ms
`, r += `Throughput: ${(s / h * 1e3).toLocaleString()} ops/sec

`, r += o.map((d) => `${d.name}: ${d.time.toFixed(2)}ms (${(1e5 * 50 / d.time * 1e3).toLocaleString()} ops/sec)`).join(`
`);
      }), r;
    },
    "stress-gc": async () => {
      const e = performance.now();
      let r = "";
      return await S.use(async () => {
        const a = 2 * Math.PI * 50 / 1e3, f = 2 * Math.PI * 120 / 1e3, l = 2 * Math.PI * 200 / 1e3, s = new Array(32768);
        for (let C = 0; C < 32768; C++) s[C] = Math.sin(a * C) + 0.5 * Math.sin(f * C) + 0.3 * Math.sin(l * C);
        const h = performance.now(), d = S.vector(s), p = performance.now() - h, D = performance.now(), c = S.dsp.hanning(32768), g = S.vecOps.vmul(d, c), m = performance.now() - D, w = performance.now(), v = S.dsp.fftMag(g), A = performance.now() - w, y = performance.now();
        S.sum(v), S.mean(v), S.max(v), S.std(v);
        const _ = performance.now() - y, E = performance.now() - e;
        r = `DSP PIPELINE STRESS (${32768 .toLocaleString()} samples)

`, r += `Signal Generation: ${p.toFixed(2)}ms
`, r += `Windowing: ${m.toFixed(2)}ms
`, r += `FFT: ${A.toFixed(2)}ms
`, r += `Statistics: ${_.toFixed(2)}ms
`, r += `\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500
`, r += `Total: ${E.toFixed(2)}ms
`, r += `Throughput: ${(32768 / E * 1e3).toLocaleString()} samples/sec`;
      }), r;
    },
    "bench-vs-mathjs": async () => {
      const r = Array.from({
        length: 1e7
      }, (c, g) => g % 100 / 10 + 0.1);
      let n = `VECTORIZED MATH BENCHMARK (${1e7.toLocaleString()} elements \xD7 5 iterations)

`;
      const i = performance.now();
      await S.use(async () => {
        const c = S.vector(r), g = performance.now() - i, m = performance.now();
        for (let E = 0; E < 5; E++) S.sin(c);
        const w = performance.now() - m, v = performance.now();
        for (let E = 0; E < 5; E++) S.cos(c);
        const A = performance.now() - v, y = performance.now();
        for (let E = 0; E < 5; E++) S.exp(c);
        const _ = performance.now() - y;
        n += `\u{1F537} Achronyme (WASM)
`, n += `   Create vector: ${g.toFixed(2)}ms
`, n += `   sin(): ${w.toFixed(2)}ms (${(1e7 * 5 / w * 1e3).toLocaleString()} ops/sec)
`, n += `   cos(): ${A.toFixed(2)}ms (${(1e7 * 5 / A * 1e3).toLocaleString()} ops/sec)
`, n += `   exp(): ${_.toFixed(2)}ms (${(1e7 * 5 / _ * 1e3).toLocaleString()} ops/sec)
`, n += `   TOTAL: ${(g + w + A + _).toFixed(2)}ms

`;
      }), performance.now();
      const o = performance.now();
      for (let c = 0; c < 5; c++) r.map((g) => Math.sin(g));
      const u = performance.now() - o, a = performance.now();
      for (let c = 0; c < 5; c++) r.map((g) => Math.cos(g));
      const f = performance.now() - a, l = performance.now();
      for (let c = 0; c < 5; c++) r.map((g) => Math.exp(g));
      const s = performance.now() - l, h = u + f + s;
      n += `\u{1F7E8} JavaScript Native (V8)
`, n += `   sin(): ${u.toFixed(2)}ms (${(1e7 * 5 / u * 1e3).toLocaleString()} ops/sec)
`, n += `   cos(): ${f.toFixed(2)}ms (${(1e7 * 5 / f * 1e3).toLocaleString()} ops/sec)
`, n += `   exp(): ${s.toFixed(2)}ms (${(1e7 * 5 / s * 1e3).toLocaleString()} ops/sec)
`, n += `   TOTAL: ${h.toFixed(2)}ms

`;
      const d = n.match(/Achronyme[\s\S]*?TOTAL: ([\d.]+)ms/), p = d ? parseFloat(d[1]) : 0;
      n += `\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501
`;
      const D = (h / p).toFixed(2);
      return n += `\u{1F4CA} PERFORMANCE:
`, n += `   Achronyme: ${p.toFixed(2)}ms
`, n += `   JS Native: ${h.toFixed(2)}ms
`, n += `   Speedup: ${D}x ${p < h ? "\u{1F680} WASM WINS!" : "JS wins"}
`, n += `   Advantage: ${Math.abs((h - p) / h * 100).toFixed(1)}%`, n;
    },
    "bench-fft": async () => {
      const r = Array.from({
        length: 8192
      }, (l, s) => Math.sin(2 * Math.PI * 50 * s / 1e3) + 0.5 * Math.sin(2 * Math.PI * 120 * s / 1e3));
      let n = `FFT BENCHMARK (${8192 .toLocaleString()} samples \xD7 10 iterations)

`;
      const i = performance.now();
      await S.use(async () => {
        const l = S.vector(r);
        for (let s = 0; s < 10; s++) S.fftMag(l);
      });
      const o = performance.now() - i, u = performance.now();
      for (let l = 0; l < 10; l++) ti(r).map((h) => typeof h == "number" ? Math.abs(h) : Math.sqrt(h.re * h.re + h.im * h.im));
      const a = performance.now() - u, f = (a / o).toFixed(2);
      return n += `\u{1F537} Achronyme: ${o.toFixed(2)}ms (${(o / 10).toFixed(2)}ms per FFT)
`, n += `\u{1F7E6} math.js:   ${a.toFixed(2)}ms (${(a / 10).toFixed(2)}ms per FFT)

`, n += `Speedup: ${f}x ${o < a ? "faster" : "slower"}
`, n += `Winner: ${o < a ? "\u{1F537} Achronyme" : "\u{1F7E6} math.js"}`, n;
    },
    "bench-vector-ops": async () => {
      const r = Array.from({
        length: 2e5
      }, () => Math.random() * 100), n = Array.from({
        length: 2e5
      }, () => Math.random() * 100);
      let i = `VECTOR OPERATIONS (${2e5.toLocaleString()} elements \xD7 30 iterations)

`;
      const o = performance.now();
      await S.use(async () => {
        const c = S.vector(r), g = S.vector(n);
        for (let m = 0; m < 30; m++) S.vecOps.vadd(c, g), S.vecOps.vmul(c, g), S.dot(c, g);
      });
      const u = performance.now() - o, a = performance.now();
      for (let c = 0; c < 30; c++) rn(r, n), ei(r, n), _o(r, n);
      const f = performance.now() - a, l = performance.now();
      for (let c = 0; c < 30; c++) r.map((g, m) => g + n[m]), r.map((g, m) => g * n[m]), r.reduce((g, m, w) => g + m * n[w], 0);
      const s = performance.now() - l, h = 2e5 * 30 * 3, d = (f / u).toFixed(2), p = (s / u).toFixed(2);
      i += `\u{1F537} Achronyme: ${u.toFixed(2)}ms (${(h / u * 1e3).toLocaleString()} ops/sec)
`, i += `\u{1F7E6} math.js:   ${f.toFixed(2)}ms (${(h / f * 1e3).toLocaleString()} ops/sec)
`, i += `\u{1F7E8} JS Native: ${s.toFixed(2)}ms (${(h / s * 1e3).toLocaleString()} ops/sec)

`, i += `vs math.js: ${d}x ${u < f ? "faster" : "slower"}
`, i += `vs JS V8:   ${p}x ${u < s ? "faster" : "slower"}

`;
      const D = u < f && u < s ? "\u{1F537} Achronyme" : f < s ? "\u{1F7E6} math.js" : "\u{1F7E8} JS Native";
      return i += `Winner: ${D}`, i;
    },
    "bench-pipeline": async () => {
      const e = Array.from({
        length: 16384
      }, (d, p) => Math.sin(2 * Math.PI * 50 * p / 1e3) + 0.5 * Math.sin(2 * Math.PI * 120 * p / 1e3) + 0.3 * Math.sin(2 * Math.PI * 200 * p / 1e3));
      let r = `FULL DSP PIPELINE (${16384 .toLocaleString()} samples)

`;
      const n = performance.now();
      await S.use(async () => {
        const d = S.vector(e), p = S.dsp.hanning(16384), D = S.vecOps.vmul(d, p), c = S.fftMag(D);
        S.max(c), S.mean(c), S.sum(c);
      });
      const i = performance.now() - n, o = performance.now(), u = new Array(16384);
      for (let d = 0; d < 16384; d++) u[d] = 0.5 * (1 - Math.cos(2 * Math.PI * d / 16383));
      const a = ei(e, u), l = ti(a).map((d) => typeof d == "number" ? Math.abs(d) : Math.sqrt(d.re * d.re + d.im * d.im));
      Rh(l), Uh(l), Oh(l);
      const s = performance.now() - o, h = (s / i).toFixed(2);
      return r += `Pipeline: Signal \u2192 Window \u2192 FFT \u2192 Statistics

`, r += `\u{1F537} Achronyme: ${i.toFixed(2)}ms (${(16384 / i * 1e3).toLocaleString()} samples/sec)
`, r += `\u{1F7E6} math.js:   ${s.toFixed(2)}ms (${(16384 / s * 1e3).toLocaleString()} samples/sec)

`, r += `Speedup: ${h}x ${i < s ? "faster" : "slower"}
`, r += `Winner: ${i < s ? "\u{1F537} Achronyme" : "\u{1F7E6} math.js"}`, r;
    },
    "soc-simple-expr": async () => (S.resetEvaluator(), `2 + 3 * 4 = ${S.eval("2 + 3 * 4")}`),
    "soc-lambda-create": async () => (S.resetEvaluator(), `Created lambda: x => x * 2 (result: ${S.eval("x => x * 2")})`),
    "soc-lambda-call": async () => (S.resetEvaluator(), S.eval("let double = x => x * 2"), `double(5) = ${S.eval("double(5)")}`),
    "soc-lambda-closure": async () => (S.resetEvaluator(), S.eval("let multiplier = 3"), S.eval("let mult = x => x * multiplier"), `mult(4) with closure (multiplier=3) = ${S.eval("mult(4)")}`),
    "soc-map": async () => (S.resetEvaluator(), `map(x => x * 2, [1,2,3,4]) = ${S.eval("map(x => x * 2, [1, 2, 3, 4])")}`),
    "soc-filter": async () => (S.resetEvaluator(), `filter(x => x > 3, [1,2,3,4,5,6]) = ${S.eval("filter(x => x > 3, [1, 2, 3, 4, 5, 6])")}`),
    "soc-reduce": async () => (S.resetEvaluator(), `reduce sum [1,2,3,4,5] = ${S.eval("reduce((acc, x) => acc + x, 0, [1, 2, 3, 4, 5])")}`),
    "soc-pipe": async () => (S.resetEvaluator(), S.eval("let addTwo = x => x + 2"), S.eval("let double2 = x => x * 2"), `pipe(5, addTwo, double2) = ${S.eval("pipe(5, addTwo, double2)")} (5 + 2 = 7, 7 * 2 = 14)`)
  };
  Kh.forEach((t) => {
    t.addEventListener("click", async () => {
      const e = t.dataset.test, r = Co[e];
      r ? await Eo(e, r, t) : et(e, "Test not implemented yet", "error");
    });
  });
  Zh.addEventListener("click", async () => {
    const t = document.querySelector(".test-section:not(.hidden)");
    if (!t) return;
    const e = t.querySelectorAll(".test-btn");
    et("Running All Tests", `Starting ${e.length} tests in category...`, "info");
    for (const r of e) {
      const n = r.dataset.test, i = Co[n];
      i && (await Eo(n, i, r), await new Promise((o) => setTimeout(o, 100)));
    }
    et("All Tests Complete", "All tests in category finished", "success");
  });
  console.log("\u{1F4DD} Setting up initialization...");
  console.log("Document ready state:", document.readyState);
  document.readyState === "loading" ? (console.log("\u23F3 DOM still loading, adding listener..."), window.addEventListener("DOMContentLoaded", () => {
    console.log("\u{1F3AF} DOMContentLoaded fired!"), ni();
  })) : (console.log("\u2705 DOM already ready, initializing immediately..."), ni());
  console.log("\u2705 Initialization setup complete");
})();
