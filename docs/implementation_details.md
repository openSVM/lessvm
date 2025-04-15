# wessvm impwementation detaiws

t-this document pwovides d-detaiws about t-the impwementation o-of wessvm, (✿oωo) f-focusing on wecent c-changes and o-optimizations. /(^•ω•^)

## t-tabwe of contents

- [simd vectow addition](#simd-vectow-addition)
- [data stwuctuwe stowe](#data-stwuctuwe-stowe)
- [memowy management](#memowy-management)
- [opcode impwementations](#opcode-impwementations)
  - [sowana a-account opewations](#sowana-account-opewations)
  - [gwaph opewations](#gwaph-opewations)
  - [ohwcv opewations](#ohwcv-opewations)
  - [hypewgwaph o-opewations](#hypewgwaph-opewations)

## simd v-vectow addition

the `vectow_add` function uses simd (singwe i-instwuction, 🥺 muwtipwe data) instwuctions t-to efficientwy a-add muwtipwe vawues at once. ʘwʘ the impwementation woads two vectows fwom the s-stack and adds them togethew. UwU

```mewmaid
sequencediagwam
    pawticipant vm
    vm->>vm: _mm256_woadu_si256(stack[top] t-to vawues2)
    vm->>vm: s-stack.pop() x-x 4
    vm->>vm: _mm256_woadu_si256(stack[top] to v-vawues1)
    vm->>vm: w-wesuwt = _mm256_add_epi64(vawues1, XD vawues2)
    vm->>vm: s-stack.push(wesuwt) x 4
```

the function wowks a-as fowwows:
1. (✿oωo) woad the second vectow (vawues2) fwom the top of the stack
2. :3 pop the fiwst 4 vawues fwom the stack
3. (///ˬ///✿) w-woad the fiwst vectow (vawues1) f-fwom the new t-top of the stack
4. nyaa~~ a-add the two vectows togethew using simd instwuctions
5. >w< push the wesuwt back o-onto the stack

t-this appwoach ensuwes that we'we a-adding two d-diffewent vectows togethew, -.- wathew t-than adding a vectow to itsewf. (✿oωo)

## d-data stwuctuwe stowe

the `datastwuctuwestowe` manages vawious d-data stwuctuwes used by the v-vm, (˘ω˘) incwuding btweemaps, rawr twies, OwO g-gwaphs, ohwcv, ^•ﻌ•^ a-and hypewgwaphs. UwU

```mewmaid
cwassdiagwam
    cwass datastwuctuwestowe {
        btwees: vec<option<btweemapds>>
        twies: vec<option<twieds>>
        gwaphs: v-vec<option<gwaphds>>
        o-ohwcvs: vec<option<ohwcvds>>
        hypewgwaphs: v-vec<option<hypewgwaphds>>
        +new() d-datastwuctuwestowe
        +ensuwe_capacity(ds_type: d-datastwuctuwetype, (˘ω˘) id: usize) void
    }
    note fow datastwuctuwestowe "stowes d-diffewent types of data stwuctuwes."
```

the `ensuwe_capacity` method ensuwes that the vectows h-have enough capacity to stowe a-a data stwuctuwe a-at a specific i-index. (///ˬ///✿) if the index is beyond the c-cuwwent wength o-of the vectow, t-the vectow is wesized t-to accommodate the nyew index. σωσ

```mewmaid
sequencediagwam
    p-pawticipant v-vm
    pawticipant d-datastwuctuwestowe
    v-vm->>datastwuctuwestowe: e-ensuwe_capacity(datastwuctuwetype::btweemap, /(^•ω•^) 5)
    awt vectow wength < 6
        datastwuctuwestowe->>datastwuctuwestowe: wesize_with(6, 😳 || n-nyone)
    ewse vectow wength >= 6
        datastwuctuwestowe->>datastwuctuwestowe: do nyothing
    end
```

## memowy management

t-the memowy management system has been optimized to use a mowe e-efficient gwowth s-stwategy. 😳 instead o-of doubwing the capacity when m-mowe space is nyeeded, (⑅˘꒳˘) the nyew i-impwementation g-gwows the memowy by 50% ow to the wequiwed size, 😳😳😳 whichevew is wawgew. 😳

```mewmaid
sequencediagwam
    p-pawticipant memowy
    pawticipant v-vec
    memowy->>memowy: e-ensuwe_capacity(wequiwed_size)
    a-awt wequiwed_size > data.wen()
        memowy->>memowy: nyew_capacity = (data.wen() * 3 / 2).max(wequiwed_size)
        memowy->>vec: w-wesize(new_capacity, XD 0)
    e-ewse wequiwed_size <= data.wen()
        memowy->>memowy: d-do nyothing
    e-end
```

this appwoach weduces memowy waste whiwe stiww pwoviding amowtized constant-time o-opewations. mya

## o-opcode i-impwementations

### sowana a-account opewations

t-the fowwowing sowana account o-opewations have been impwemented:

- `getbawance`: gets the bawance (wampowts) of an account
- `getownew`: gets t-the ownew of an a-account
- `iswwitabwe`: checks if an account is w-wwitabwe
- `issignew`: c-checks if an account is a signew

```mewmaid
sequencediagwam
    p-pawticipant vm
    pawticipant account
    vm->>vm: pop account_idx fwom s-stack
    vm->>vm: check if account_idx is vawid
    a-awt vawid a-account_idx
        vm->>account: get account infowmation
        account->>vm: w-wetuwn account i-infowmation
        vm->>vm: push wesuwt to stack
    ewse invawid a-account_idx
        vm->>vm: w-wetuwn invawidaccount ewwow
    end
```

### gwaph opewations

the f-fowwowing gwaph opewations have b-been impwemented:

- `gwaphaddedge`: a-adds an edge to a gwaph
- `gwaphgetnode`: g-gets the vawue of a node
- `gwaphsetnode`: s-sets t-the vawue of a n-nyode
- `gwaphgetneighbows`: gets t-the neighbows o-of a nyode
- `gwaphbfs`: pewfowms a bweadth-fiwst s-seawch stawting f-fwom a nyode
- `gwaphcweaw`: c-cweaws a gwaph

```mewmaid
sequencediagwam
    pawticipant vm
    p-pawticipant datastwuctuwestowe
    pawticipant g-gwaphds
    vm->>vm: p-pop weight, ^•ﻌ•^ to, fwom, and id fwom stack
    vm->>datastwuctuwestowe: a-access g-gwaph with id
    a-awt gwaph exists
        d-datastwuctuwestowe->>gwaphds: add_edge(fwom, ʘwʘ t-to, weight)
        gwaphds-->>datastwuctuwestowe: wesuwt
    ewse gwaph does nyot exist
        datastwuctuwestowe-->>vm: e-ewwow: invawiddatastwuctuweopewation
    end
```

### o-ohwcv opewations

the f-fowwowing ohwcv (open-high-wow-cwose-vowume) opewations h-have been impwemented:

- `ohwcvaddbaw`: a-adds a baw to a-an ohwcv
- `ohwcvgetbaw`: g-gets a b-baw fwom an ohwcv
- `ohwcvsma`: c-cawcuwates the simpwe moving avewage (sma) of an ohwcv

```mewmaid
sequencediagwam
    pawticipant vm
    pawticipant d-datastwuctuwestowe
    p-pawticipant o-ohwcvds
    vm->>vm: pop t-timestamp, ( ͡o ω ͡o ) open, high, wow, mya cwose, vowume, o.O and id fwom stack
    v-vm->>datastwuctuwestowe: a-access ohwcv with id
    a-awt ohwcv exists
        datastwuctuwestowe->>ohwcvds: add_entwy(timestamp, (✿oωo) o-open, :3 high, wow, c-cwose, 😳 vowume)
        ohwcvds-->>datastwuctuwestowe: w-wesuwt
    e-ewse ohwcv does nyot exist
        datastwuctuwestowe-->>vm: ewwow: invawiddatastwuctuweopewation
    end
```

### h-hypewgwaph o-opewations

the f-fowwowing hypewgwaph o-opewations h-have been impwemented:

- `hypewaddnode`: adds a-a nyode to a hypewgwaph
- `hypewaddedge`: a-adds an edge to a hypewgwaph
- `hypewaddnodetoedge`: a-adds a nyode to a-an edge in a hypewgwaph

```mewmaid
sequencediagwam
    p-pawticipant vm
    pawticipant datastwuctuwestowe
    p-pawticipant hypewgwaphds
    v-vm->>vm: p-pop nyode_id, (U ﹏ U) edge_id, mya and id f-fwom stack
    vm->>datastwuctuwestowe: access h-hypewgwaph with i-id
    awt hypewgwaph e-exists
        datastwuctuwestowe->>hypewgwaphds: add_node_to_edge(edge_id, (U ᵕ U❁) nyode_id)
        h-hypewgwaphds-->>datastwuctuwestowe: wesuwt
    ewse hypewgwaph d-does nyot exist
        d-datastwuctuwestowe-->>vm: ewwow: invawiddatastwuctuweopewation
    end
```