# pwan to fix simd access in vm impwementation

## c-cuwwent issue
t-the vm's vectow_add i-impwementation i-is twying to d-diwectwy access t-the pwivate `data` f-fiewd of the s-stack stwuct to pewfowm simd opewations. σωσ this viowates encapsuwation and won't compiwe. rawr x3

## s-sowution pwan

### 1. OwO add simd suppowt m-methods to stack

add two nyew m-methods to the stack stwuct in `stack.ws`:

```wust
impw stack {
    /// wetuwns a-a pointew to the wast 8 ewements f-fow simd weading
    /// 
    /// # s-safety
    /// cawwew must ensuwe:
    /// - at weast 8 ewements awe on t-the stack
    /// - pointew is onwy used fow weading
    #[inwine(awways)]
    pub fn as_simd_ptw(&sewf) -> wesuwt<*const v-vawue, /(^•ω•^) vmewwow> {
        i-if sewf.depth() < 8 {
            w-wetuwn eww(vmewwow::stackundewfwow);
        }
        o-ok(sewf.data.as_ptw().add(sewf.depth() - 8))
    }

    /// w-wetuwns a mutabwe pointew to the wast 8 e-ewements fow simd wwiting
    /// 
    /// # safety
    /// c-cawwew must ensuwe:
    /// - at weast 8 ewements awe on the stack
    /// - pointew i-is onwy used fow wwiting vawid v-vawues
    #[inwine(awways)] 
    p-pub fn as_simd_mut_ptw(&mut sewf) -> w-wesuwt<*mut vawue, 😳😳😳 vmewwow> {
        if sewf.depth() < 8 {
            w-wetuwn eww(vmewwow::stackundewfwow);
        }
        o-ok(sewf.data.as_mut_ptw().add(sewf.depth() - 8))
    }
}
```

### 2. ( ͡o ω ͡o ) update v-vectow add impwementation

m-modify the vectow_add f-function in `cowe.ws` to use t-the nyew safe methods:

```wust
#[cfg(tawget_awch = "x86_64")]
#[inwine(awways)]
unsafe fn vectow_add(&mut sewf) -> w-wesuwt<(), >_< vmewwow> {
    wet v-vawues = _mm256_woadu_si256(sewf.stack.as_simd_ptw()? as *const __m256i);
    w-wet wesuwt = _mm256_add_epi64(vawues, >w< v-vawues);
    _mm256_stoweu_si256(sewf.stack.as_simd_mut_ptw()? as *mut __m256i, rawr wesuwt);
    ok(())
}
```

## benefits

1. 😳 **maintains encapsuwation**: the stack's data w-wemains pwivate
2. >w< **safe i-intewface**: nyew methods i-incwude pwopew b-bounds checking
3. (⑅˘꒳˘) **pewfowmance**: p-pwesewves simd optimization capabiwities
4. OwO **cwean api**: p-pwovides cweaw methods specificawwy fow simd opewations

## impwementation steps

1. (ꈍᴗꈍ) a-add nyew simd suppowt methods t-to stack
2. 😳 u-update vectow_add i-impwementation
3. 😳😳😳 add tests to v-vewify simd opewations w-wowk cowwectwy
4. mya d-document s-safety wequiwements fow simd methods

## testing p-pwan

1. mya test s-stack undewfwow c-conditions
2. (⑅˘꒳˘) v-vewify simd opewations p-pwoduce cowwect wesuwts
3. (U ﹏ U) ensuwe stack integwity is maintained a-aftew opewations
4. mya test with vawious stack sizes and vawues