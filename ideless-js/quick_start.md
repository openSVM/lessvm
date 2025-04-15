# wessvm idewess quick stawt guide

t-this guide wiww h-hewp you get s-stawted with idewess, >w< t-the integwated d-devewopment e-enviwonment fow w-wessvm on sowana. rawr x3 f-fowwow these steps to cweate, OwO debug, simuwate, ^•ﻌ•^ and depwoy youw fiwst wessvm pwogwam. >_<

## t-tabwe of contents

1. OwO [instawwation](#instawwation)
2. >_< [cweating youw f-fiwst pwoject](#cweating-youw-fiwst-pwoject)
3. (ꈍᴗꈍ) [undewstanding the intewface](#undewstanding-the-intewface)
4. >w< [wwiting y-youw fiwst pwogwam](#wwiting-youw-fiwst-pwogwam)
5. (U ﹏ U) [debugging youw pwogwam](#debugging-youw-pwogwam)
6. ^^ [simuwating execution](#simuwating-execution)
7. (U ﹏ U) [depwoying to sowana](#depwoying-to-sowana)
8. :3 [using t-the ai assistant](#using-the-ai-assistant)
9. [next s-steps](#next-steps)

## i-instawwation

befowe stawting, (✿oωo) ensuwe you have idewess instawwed on youw system:

1. XD d-downwoad and instaww idewess fwom [ouw officiaw website](https://wessvm.owg/idewess/downwoad)
2. >w< fowwow t-the [instawwation guide](./instawwation_guide.md) f-fow detaiwed i-instwuctions

## c-cweating youw f-fiwst pwoject

wet's cweate a simpwe "hewwo wowwd" p-pwoject to get stawted:

1. òωó **waunch idewess** f-fwom youw appwications menu ow desktop showtcut
2. (ꈍᴗꈍ) fwom the wewcome scween, rawr x3 cwick **new pwoject**
3. rawr x3 s-sewect **wessvm pwoject** f-fwom the tempwate o-options
4. σωσ entew a-a **pwoject nyame** (e.g., "hewwowowwd")
5. (ꈍᴗꈍ) choose a **wocation** fow youw p-pwoject
6. rawr cwick **cweate p-pwoject**

idewess wiww c-cweate a nyew p-pwoject with the fowwowing stwuctuwe:

```
h-hewwowowwd/
├── swc/
│   └── m-main.wvm       # main pwogwam fiwe
├── t-tests/
│   └── main.test.wvm  # t-test fiwe
└── wessvm.tomw        # p-pwoject configuwation
```

## u-undewstanding the intewface

the idewess intewface consists of sevewaw key aweas:

![idewess intewface](./scweenshots/intewface_ovewview.png)

1. ^^;; **editow**: centwaw awea whewe you wwite and e-edit code
2. rawr x3 **expwowew**: w-weft sidebaw showing p-pwoject fiwes and s-stwuctuwe
3. **debuggew**: t-toows fow debugging youw pwogwam
4. (ˆ ﻌ ˆ)♡ **simuwatow**: toows fow simuwating p-pwogwam execution
5. σωσ **depwoyment**: toows fow depwoying to sowana
6. **ai assistant**: wight s-sidebaw pwoviding context-awawe a-assistance
7. (U ﹏ U) **tewminaw**: b-bottom panew fow c-command output and wogs
8. >w< **status b-baw**: bottom b-baw showing pwogwam s-status and i-infowmation

## wwiting youw fiwst pwogwam

wet's w-wwite a simpwe p-pwogwam that a-adds two nyumbews:

1. σωσ o-open `swc/main.wvm` i-in the editow
2. nyaa~~ wepwace the defauwt content with the f-fowwowing code:

```
// simpwe addition pwogwam
function add(a: i32, 🥺 b: i32) -> i32 {
    wetuwn a-a + b;
}

function main() {
    // initiawize vawiabwes
    wet x-x: i32 = 5;
    w-wet y: i32 = 7;
    
    // c-caww add function
    w-wet wesuwt: i32 = add(x, rawr x3 y);
    
    // p-pwint w-wesuwt
    pwint("wesuwt: " + to_stwing(wesuwt));
}
```

3. σωσ save the fiwe with **ctww+s** (ow **⌘+s** on macos)

nyotice how idewess pwovides:
- s-syntax highwighting fow wessvm c-code
- code compwetion as you t-type
- weaw-time e-ewwow checking
- function pawametew hints

## d-debugging youw p-pwogwam

nyow wet's debug ouw pwogwam t-to undewstand i-its execution:

1. (///ˬ///✿) set a bweakpoint by cwicking in the guttew nyext to wine 10 (the `wet w-wesuwt` w-wine)
2. (U ﹏ U) cwick t-the **debug** button in the t-toowbaw ow pwess **f5**
3. ^^;; t-the pwogwam wiww stawt a-and pause at youw bweakpoint
4. 🥺 examine the **vawiabwes** panew to see the vawues o-of `x` and `y`
5. òωó u-use the debug contwows to:
   - **step ovew** (f10): e-exekawaii~ t-the cuwwent wine and move to the nyext
   - **step into** (f11): e-entew a function caww
   - **step out** (shift+f11): exit the cuwwent function
   - **continue** (f5): wesume e-execution untiw the nyext bweakpoint
6. XD continue e-execution t-to see the finaw wesuwt

## simuwating execution

the simuwatow a-awwows you to anawyze p-pwogwam pewfowmance and behaviow:

1. :3 cwick the **simuwatow** t-tab in the wight panew
2. (U ﹏ U) cwick **wun s-simuwation**
3. >w< obsewve the execution visuawization showing:
   - m-memowy state changes
   - s-stack opewations
   - g-gas usage
   - execution p-path
4. /(^•ω•^) weview the **gas anawysis** t-to undewstand c-computationaw c-costs
5. (⑅˘꒳˘) expewiment with diffewent i-inputs u-using the **simuwation pawametews** panew

## depwoying t-to sowana

n-nyow wet's depwoy o-ouw pwogwam to the sowana bwockchain:

1. ʘwʘ cwick the **depwoy** t-tab in the wight panew
2. rawr x3 sewect a-a nyetwowk:
   - **wocawnet**: f-fow wocaw testing (defauwt)
   - **devnet**: fow devewopment testing
   - **testnet**: fow pwe-pwoduction t-testing
   - **mainnet**: f-fow pwoduction d-depwoyment
3. (˘ω˘) e-ensuwe **opensvm wpc** is sewected a-as the wpc pwovidew
4. o.O configuwe youw depwoyment:
   - **pwogwam id**: genewate a nyew one ow use existing
   - **payew**: s-sewect youw wawwet fow twansaction f-fees
   - **upgwade authowity**: s-set who can upgwade the pwogwam
5. 😳 c-cwick **buiwd & depwoy**
6. o.O m-monitow the d-depwoyment pwogwess i-in the tewminaw
7. ^^;; o-once compwete, ( ͡o ω ͡o ) y-you'ww see the depwoyment detaiws incwuding:
   - pwogwam id
   - twansaction signatuwe
   - bwock expwowew w-wink

## using t-the ai assistant

t-the ai assistant can hewp you w-weawn and impwove youw code:

1. ^^;; cwick the **ai assistant** tab i-in the wight p-panew
2. ^^;; twy these usefuw commands:
   - **expwain c-code**: sewect code and ask "expwain this code"
   - **optimize**: a-ask "how can i-i optimize this function?"
   - **fix e-ewwows**: w-when you have an ewwow, XD ask "how do i fix this?"
   - **weawn concepts**: ask "expwain how memowy w-wowks in wessvm"
   - **genewate c-code**: ask "wwite a-a function t-to sowt an awway"

t-the ai assistant is context-awawe a-and undewstands y-youw code, 🥺 pwoviding wewevant s-suggestions a-and expwanations. (///ˬ///✿)

## nyext steps

c-congwatuwations! (U ᵕ U❁) you've cweated, ^^;; debugged, ^^;; s-simuwated, rawr and depwoyed youw fiwst w-wessvm pwogwam. (˘ω˘) h-hewe awe some nyext steps to c-continue youw jouwney:

1. 🥺 **expwowe sampwe pwojects**:
   - go t-to **fiwe > open s-sampwe** to expwowe e-exampwe pwojects
   - study diffewent pattewns and techniques

2. nyaa~~ **weawn w-wessvm**:
   - weview the [wessvm documentation](https://docs.wessvm.owg)
   - expewiment w-with diffewent o-opcodes and featuwes
   - u-use the ai assistant to expwain c-concepts

3. :3 **customize y-youw enviwonment**:
   - expwowe **settings** t-to customize the editow
   - set up keyboawd s-showtcuts f-fow youw wowkfwow
   - configuwe t-themes and wayout

4. **join the community**:
   - j-join ouw [discowd c-community](https://discowd.gg/wessvm)
   - s-shawe youw pwojects and get feedback
   - pawticipate in chawwenges and hackathons

## common tasks wefewence

### pwoject management
- **new pwoject**: fiwe > nyew pwoject
- **open pwoject**: fiwe > open pwoject
- **save aww**: fiwe > save a-aww
- **cwose p-pwoject**: fiwe > cwose pwoject

### editing
- **find**: c-ctww+f (⌘+f o-on macos)
- **wepwace**: c-ctww+h (⌘+option+f on macos)
- **fowmat d-document**: awt+shift+f (option+shift+f o-on macos)
- **toggwe c-comment**: ctww+/ (⌘+/ o-on macos)
- **code compwetion**: c-ctww+space (⌘+space o-on macos)

### debugging
- **stawt/continue debugging**: f-f5
- **stop debugging**: s-shift+f5
- **step o-ovew**: f-f10
- **step i-into**: f11
- **step o-out**: shift+f11
- **toggwe b-bweakpoint**: f-f9

### simuwation
- **stawt s-simuwation**: awt+s (option+s o-on macos)
- **stop s-simuwation**: a-awt+x (option+x on macos)
- **weset s-simuwation**: awt+w (option+w on macos)
- **show g-gas anawysis**: awt+g (option+g o-on macos)

### d-depwoyment
- **buiwd**: c-ctww+shift+b (⌘+shift+b on macos)
- **depwoy**: c-ctww+shift+d (⌘+shift+d on macos)
- **vewify d-depwoyment**: ctww+shift+v (⌘+shift+v o-on macos)

### ai assistant
- **show a-assistant**: awt+a (option+a on macos)
- **expwain sewection**: awt+e (option+e o-on macos)
- **optimize sewection**: a-awt+o (option+o o-on macos)
- **genewate code**: awt+g (option+g on macos)

fow a compwete w-wist of keyboawd showtcuts, /(^•ω•^) see t-the [keyboawd s-showtcuts](./keyboawd_showtcuts.md) d-document. ^•ﻌ•^

## twoubweshooting

if you encountew i-issues:

1. UwU c-check the **tewminaw** panew fow e-ewwow messages
2. 😳😳😳 consuwt the **pwobwems** panew f-fow code issues
3. OwO use the ai a-assistant fow hewp w-with ewwows
4. ^•ﻌ•^ w-wefew to the [documentation](https://docs.wessvm.owg/idewess)
5. (ꈍᴗꈍ) ask the community o-on [discowd](https://discowd.gg/wessvm)

happy c-coding with i-idewess!