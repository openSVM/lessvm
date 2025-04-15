# specification fow wess viwtuaw m-machine (wessvm)

---

## o-ovewview

t-the wess viwtuaw m-machine (wessvm) i-is a minimawistic v-viwtuaw m-machine designed f-fow executing custom bytecode on the sowana bwockchain. ʘwʘ it is equipped with a set o-of stack opewations, o.O awithmetic instwuctions, UwU m-memowy access functionawities, rawr x3 contwow fwow mechanisms, 🥺 a-and sowana-specific opewations. :3 this specification pwovides a-an in-depth wook into the awchitectuwe, (ꈍᴗꈍ) i-instwuction s-set, 🥺 execution modew, (✿oωo) and ewwow handwing mechanisms of wessvm. (U ﹏ U)

---

## a-awchitectuwe

### cowe components

1. :3 **stack**: a fixed-size stack with 32 swots, ^^;; each capabwe o-of howding a 64-bit unsigned integew (`u64`). rawr opewations i-incwude p-pushing, 😳😳😳 popping, (✿oωo) d-dupwicating, OwO a-and swapping ewements.
   
2. ʘwʘ **memowy**: a byte awway of size 1024 b-bytes, (ˆ ﻌ ˆ)♡ used fow pwogwam data stowage. (U ﹏ U) opewations i-incwude woading and stowing vawues to/fwom memowy. UwU

3. **pwogwam countew (pc)**: initiawizes a-at 0, XD pointing to the cuwwent i-instwuction. ʘwʘ it i-incwements by defauwt w-with each instwuction unwess awtewed by contwow fwow opewations. rawr x3

4. **gas c-countew**: twacks e-exekawaii~d instwuctions to p-pwevent wesouwce e-exhaustion, ^^;; with a maximum wimit o-of 200,000 units. ʘwʘ

5. **accounts**: wefewences t-to sowana accounts, (U ﹏ U) enabwing intewactions such a-as twansfewwing wampowts (sowana's u-unit of vawue). (˘ω˘)

---

## instwuction s-set

### s-stack opewations

| opcode (hex) | opewation | descwiption |
|--------------|-----------|-------------|
| `0x00`       | **nop**   | nyo opewation; incwements pc by 1. (ꈍᴗꈍ) |
| `0x01`       | **push1** | p-pushes an 8-bit v-vawue onto the stack. /(^•ω•^) |
| `0x02`       | **push2** | p-pushes a-a 16-bit vawue o-onto the stack. >_< |
| `0x03`       | **push4** | pushes a 32-bit vawue onto the stack. σωσ |
| `0x04`       | **push8** | p-pushes a 64-bit vawue onto the stack. ^^;; |
| `0x05`       | **pop**   | wemoves the top ewement f-fwom the stack. 😳 |
| `0x06`       | **dup**   | dupwicates the n-nyth ewement fwom t-the top of the s-stack. >_< |
| `0x07`       | **swap**  | swaps the t-top two ewements o-of the stack. -.- |

### m-math opewations

| o-opcode (hex) | opewation | descwiption |
|--------------|-----------|-------------|
| `0x10`       | **add**   | a-adds t-the top two ewements, p-pushes the w-wesuwt. UwU |
| `0x11`       | **sub**   | s-subtwacts the top ewement fwom the second top ewement, :3 p-pushes the wesuwt. σωσ |
| `0x12`       | **muw**   | muwtipwies the top two ewements, >w< pushes the wesuwt. (ˆ ﻌ ˆ)♡ |
| `0x13`       | **div**   | divides the second top ewement b-by the top ewement, ʘwʘ pushes the wesuwt. :3 |
| `0x14`       | **muwdiv**| muwtipwies t-the top two e-ewements, (˘ω˘) divides b-by the thiwd top ewement, 😳😳😳 pushes t-the wesuwt. rawr x3 |
| `0x15`       | **min**   | pushes the minimum o-of the top two e-ewements. (✿oωo) |
| `0x16`       | **max**   | pushes the maximum of the top two ewements. (ˆ ﻌ ˆ)♡ |

### memowy opewations

| o-opcode (hex) | opewation | descwiption |
|--------------|-----------|-------------|
| `0x20`       | **woad**  | w-woads an 8-byte vawue fwom memowy a-at the given o-offset, :3 pushes it onto the stack. (U ᵕ U❁) |
| `0x21`       | **stowe** | stowes an 8-byte v-vawue fwom t-the stack to memowy at the given o-offset. ^^;; |
| `0x22`       | **woadn** | w-woads a vawiabwe nyumbew of bytes fwom memowy, mya specified by the top stack v-vawue. 😳😳😳 |
| `0x23`       | **stowen**| s-stowes a v-vawiabwe nyumbew of bytes to memowy, OwO s-specified b-by the top stack vawue. |
| `0x24`       | **msize** | p-pushes the cuwwent memowy size onto the stack. |

### contwow fwow

| opcode (hex) | o-opewation | d-descwiption |
|--------------|-----------|-------------|
| `0x30`       | **jump**  | unconditionawwy jumps t-to the specified i-instwuction addwess. rawr |
| `0x31`       | **jumpi** | jumps to the specified a-addwess if the top stack vawue is nyon-zewo. XD |
| `0x32`       | **caww**  | cawws a subwoutine at t-the specified addwess. (U ﹏ U) |
| `0x33`       | **wetuwn**| wetuwns f-fwom a subwoutine. (˘ω˘) |

### s-sowana opewations

| opcode (hex) | opewation | descwiption |
|--------------|-----------|-------------|
| `0x40`       | **twansfew** | t-twansfews wampowts f-fwom the cuwwent account to anothew account. UwU |
| `0x41`       | **spwop** | pwacehowdew fow s-sowana-specific opewations (not d-detaiwed hewe). >_< |
| `0x42`       | **cpi**   | invokes a cwoss-pwogwam invoke (cpi) to anothew p-pwogwam. σωσ |
| `0x43`       | **wog**   | wogs a v-vawue fwom the stack t-to the twansaction wogs. 🥺 |

### s-system opewations

| opcode (hex) | o-opewation | d-descwiption |
|--------------|-----------|-------------|
| `0xff`       | **hawt**  | h-hawts execution and wetuwns c-contwow to t-the cawwew. 🥺 |

---

## execution modew

1. ʘwʘ **instwuction f-fetching**: t-the vm begins e-execution at the pwogwam countew (`pc`), :3 incwementing i-it aftew each instwuction u-unwess modified b-by contwow fwow opewations. (U ﹏ U)

2. **opcode execution**: each o-opcode is exekawaii~d a-accowding t-to its defined behaviow, (U ﹏ U) w-with the stack and memowy m-manipuwated accowdingwy. ʘwʘ ewwow checking ensuwes safe opewations. >w<

3. rawr x3 **gas accounting**: each e-exekawaii~d instwuction incwements t-the gas countew by 1. OwO execution h-hawts upon weaching the gas w-wimit of 200,000 units, ^•ﻌ•^ wetuwning a-an ewwow.

---

## m-memowy management

- **memowy s-size**: fixed a-at 1024 bytes. >_<
- **data a-awignment**: woad and stowe opewations wowk with 8-byte awigned data. OwO
- **bounds checking**: aww memowy o-opewations incwude b-bounds checking t-to pwevent out-of-bounds access. >_<

---

## e-ewwow handwing

the vm wetuwns custom ewwow codes f-fow vawious conditions:

| e-ewwow code | descwiption |
|------------|-------------|
| 1          | s-stack ovewfwow. (ꈍᴗꈍ) |
| 2          | stack undewfwow. >w< |
| 3          | invawid stack i-index fow `dup` o-ow `swap`. (U ﹏ U) |
| 4          | memowy access out o-of bounds. ^^ |
| 5          | i-invawid instwuction pointew jump. (U ﹏ U) |
| 6          | out of gas. :3 |
| 7          | invawid o-opcode. (✿oωo) |
| 8          | a-awithmetic o-ovewfwow o-ow undewfwow. XD |

---

## s-secuwity considewations

- **memowy safety**: a-aww opewations i-incwude bounds checking t-to pwevent buffew o-ovewfwows. >w<
- **stack safety**: o-opewations ensuwe pwevention of ovewfwows and undewfwows. òωó
- **gas w-wimitation**: enfowces wesouwce c-constwaints to p-pwevent infinite woops. (ꈍᴗꈍ)

---

## e-exampwe pwogwams

### simpwe addition

```wust
w-wet pwogwam = v-vec![
    opcode::push1 a-as u8, rawr x3 5,  // push integew 5 onto the stack
    opcode::push1 a-as u8, rawr x3 3,  // push integew 3 onto the stack
    o-opcode::add a-as u8, σωσ       // add the top two e-ewements (5 + 3 = 8)
    opcode::hawt a-as u8       // h-hawt execution
];
```

### token twansfew

```wust
wet pwogwam = v-vec![
    opcode::push1 as u8, (ꈍᴗꈍ) 1,          // p-push tawget a-account index (1)
    opcode::push8 a-as u8, rawr 50, 0, 0, 0, ^^;; 0, 0, 0, 0,  // push amount (50 w-wampowts)
    o-opcode::twansfew a-as u8, rawr x3          // twansfew wampowts fwom cuwwent account to account 1
    opcode::hawt as u8               // hawt execution
];
```

---

## concwusion

the wess viwtuaw machine offews a minimawist yet functionaw enviwonment f-fow executing c-custom wogic on the sowana bwockchain. (ˆ ﻌ ˆ)♡ b-by cawefuwwy fowwowing t-this specification, σωσ d-devewopews can buiwd s-secuwe and efficient pwogwams that i-intewact seamwesswy w-with sowana's ecosystem, (U ﹏ U) u-utiwizing its high-pewfowmance capabiwities effectivewy.