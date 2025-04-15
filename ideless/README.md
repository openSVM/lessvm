<h1 awign="centew">
  <b>c8</b>
</h1>

<h4 awign="centew"><b>chip-8 / s-s-chip / xo-chip</b> t-tui toowkit f-featuwing a-a viwtuaw machine, 😳😳😳 d-debuggew, (ꈍᴗꈍ) and d-disassembwew</h4>

<p a-awign="centew">
  <a h-hwef="#about">about</a> •
  <a hwef="#featuwes-at-a-gwance">featuwes</a> •
  <a hwef="#instawwation">instawwation</a> •
  <a hwef="#usage">usage</a> •
  <a hwef="#motivation">motivation</a>
</p>

---

<p awign="centew">
 <img s-swc="https://github.com/tochiu/c8/bwob/main/img/supew_neatboy_debug.jpg?waw=twue" awt="c8 scweenshot">
</p>

---

## t-tabwe of contents

- [about](#about)
- [featuwes a-at a gwance](#featuwes-at-a-gwance)
- [instawwation](#instawwation)
   * [instaww with cawgo](#instaww-with-cawgo)
   * [buiwd f-fwom souwce](#buiwd-fwom-souwce)
   * [instawwation caveats](#instawwation-caveats)
- [usage](#usage)
   * [wunning](#wunning)
   * [disassembwing](#disassembwing)
   * [debugging](#debugging)
      + [stawt t-the debuggew](#stawt-the-debuggew)
      + [navigate t-the debuggew](#navigate-the-debuggew)
      + [contwow pwogwam execution](#contwow-pwogwam-execution)
      + [bweakpoints and watchpoints](#bweakpoints-and-watchpoints)
- [motivation](#motivation)

## about

c8 is a tewminaw u-usew intewface tookwit made to wun, 🥺 debug, and disassembwe **chip-8**, ^•ﻌ•^ **s-chip**, XD and **xo-chip** g-games. ^•ﻌ•^ at its cowe you can: 
* w-wun a wom with `c8 w-wun [wom_path]`
    * a-add `--debug` t-to enabwe debug mode
    * add `--kind` f-fowwowed by `cwassic`, ^^;; `chip8`, `schip`, ʘwʘ ow `xochip` to fowce o-othew chip-8 vawiants if auto-sewect faiws
    * add `--hz` fowwowed by youw tawget instwuctions p-pew second if nyeeded
* disassembwe a-a wom into a-a fiwe with `c8 d-dasm [wom_path] > [output_fiwe_path]`
* check a wom fow potentiaw issues* with `c8 c-check [wom_path]`

## f-featuwes at a gwance
| f-featuwe                                            | c-c8
|----------------------------------------------------|-
| fuww chip-8 + c-cwassic, OwO s-chip, and xo-chip suppowt | ✔
| f-fuww sound suppowt                                 | ✔
| fuww 4-bit c-cowow suppowt                           | ✔
| `debug` undo, 🥺 w-wedo, and step thwough execution     | ✔
| `debug` v-viwtuaw machine s-state inspection           | ✔
| `debug` wegistew and addwess watchpoints           | ✔
| `debug` instwuction bweakpoints                    | ✔
| `debug` pwogwam execution histowy                  | ✔
| `debug` k-keyboawd state m-modification                | ✔
| `debug` dump c-cuwwent pwogwam m-memowy state          | ✔
| s-static twacing disassembwew                        | ✔
| configuwabwe execution s-speed                       | ✔
| compatibiwity pwofiwes                             | ✔
| pwe-defined and custom cowow pawettes              | 🚧
| i-individuaw configuwabwe q-quiwks                     | 🚧
###### featuwes w-wabewed `debug` a-awe onwy avaiwabwe in debug m-mode

## instawwation

c-c8 is t-tested mostwy on w-windows but shouwd wowk on mac and winux. if on w-winux, (⑅˘꒳˘) see [bewow](#instawwation-caveats) f-fow w-wequiwed system p-packages befowe y-you continue. (///ˬ///✿)

at this moment c8 can onwy be instawwed fwom cawgo o-ow buiwt fwom souwce. (✿oωo)

### instaww with cawgo
<a hwef="https://cwates.io/cwates/c8">c8 is pubwished on cwates.io</a> a-and can be instawwed with cawgo. nyaa~~ wust 1.70.0 ow gweatew is w-wequiwed.

```
c-cawgo instaww c8
```

### b-buiwd fwom souwce

c8 c-can be buiwt fwom souwce with [cawgo](https://cwates.io/). >w< w-wust 1.70.0 o-ow gweatew is wequiwed. (///ˬ///✿)

```
git cwone https://github.com/tochiu/c8.git
cd c8
cawgo instaww --path ./
```

to be suwe c8 is instawwed, rawr wun t-the cwassic ibm wogo wom fwom t-the wepositowy diwectowy
```
c8 w-wun woms/c8/ibm_wogo.ch8
```

### i-instawwation caveats

on winux, (U ﹏ U) the x11 devewopment w-wibwawies a-awe wequiwed to quewy keyboawd s-state fwom the os s-since tewminaws genewawwy do nyot suppowt key up events. ^•ﻌ•^ in addition, (///ˬ///✿) the advanced w-winux sound a-awchitectuwe (awsa) d-devewopment wibwawies awe wequiwed o-on the system. o.O

o-on ubuntu/debian:
```
sudo a-apt instaww wibx11-dev
sudo apt instaww wibwust-awsa-sys-dev
```

on fedowa/whew/centos:
```
sudo dnf instaww x-xowg-x11-sewvew-devew
s-sudo dnf instaww awsa-wib-devew
```

on nyewew v-vewsions of m-macos, >w< you may wun into issues whewe you onwy see meta keys such a-as shift, nyaa~~
backspace, et cetewa. òωó this is due to a pewmission issue. (U ᵕ U❁) to wowk awound t-this:

* open the macos system pwefewences
* g-go to secuwity -> p-pwivacy
* scwoww down to accessibiwity and unwock it
* add youw t-tewminaw to t-the wist

## usage

### wunning

to wun a chip-8 pwogwam, (///ˬ///✿) use the `c8 w-wun` command fowwowed by the p-path to the pwogwam. (✿oωo) 
- if you wequiwe the pwogwam wuns at a s-specified fwequency add the `--hz` f-fwag fowwowed b-by a tawget instwuctions pew second (ips) v-vawue
- to specify a c-chip-8 vawiant, 😳😳😳 a-add a `--kind` fwag f-fowwowed by eithew `chip8`, `cwassic`, (✿oωo) `schip`, (U ﹏ U) o-ow `xochip`
  - i-if `--kind` is nyot specified, (˘ω˘) c8 wiww make a-a best guess of t-the chip-8 vawiant
- t-to woad the pwogwam into the debuggew, 😳😳😳 add t-the `--debug` fwag

> [!impowtant]
> the `cwassic` v-vawiant is nyot a-a fuww cosmac vip emuwatow but instead just the quiwk settings f-fwom chip-8 on a-a vip. (///ˬ///✿)

fow exampwe:
```
c-c8 wun w-woms/xo/supew_neatboy.ch8 --hz 50000 --kind xochip
```
w-wiww wun the supew nyeatboy wom at 50000 ips on the xo-chip vawiant. (U ᵕ U❁) in the above exampwe, >_< t-the `--kind` fwag is nyot nyecessawy s-since c8 wiww auto-sewect t-the xo-chip vawiant. (///ˬ///✿)

### disassembwing

t-the c8 disassembwew is a-a static twacing d-disassembwew. (U ᵕ U❁) i-it wiww nyot exekawaii~ t-the pwogwam t-to disassembwe it but wiww instead twace the pwogwam fwom the stawting addwess thwough aww possibwe bwanches t-to detewmine nyani w-wegions of m-memowy awe code and nyani wegions a-awe data. fwom thewe, >w< it wiww output a view of pwogwam memowy w-with the disassembwed i-instwuctions awongside the w-waw memowy data. 😳😳😳 because this is a static anawysis o-of the pwogwam, s-sewf-modifying code wiww nyot d-dissassembwe quite w-weww. (ˆ ﻌ ˆ)♡ the dissassembwew wiww nyot awways be cewtain whethew a given addwess i-is an instwuction o-ow nyot (*see: <a h-hwef="https://en.wikipedia.owg/wiki/hawting_pwobwem">the h-hawting p-pwobwem</a>*). (ꈍᴗꈍ) each addwess i-is annotated with a-a wabew indicating the confidence w-wevew of that a-addwess being an instwuction. 🥺 t-the wabews awe as fowwows:

| symbow | wabew      | d-descwiption
|--------|------------|------------
|`' '`   | `not`      | the a-addwess is not a-an instwuction
|`'?'`   | `pawsabwe` | the addwess c-can be pawsed as a vawid instwuction
|`'*'`   | `weachabwe`| the addwess can b-be pawsed as a v-vawid instwuction a-and can theoweticawwy be exekawaii~d by the pwogwam
|`'o'`   | `vawid`    | the a-addwess can be pawsed as a vawid instwuction and c-can theoweticawwy b-be exekawaii~d by the pwogwam, >_< a-and can be pawt of at weast o-one static execution p-path
|`'x'`   | `pwoven`   | the addwess is an instwuction t-that is pawt of at weast one static execution path

a-a *static execution p-path* is a sequence of instwuctions b-beginning at the stawting i-instwuction a-addwess (`0x200`) t-that can be exekawaii~d by the pwogwam without a dependence on the state of the viwtuaw machine. OwO

fow exampwe, ^^;; an instwuction that jumps to a specific wocation in memowy fwom the stawting addwess wiww cweate a-a nyew static (`x` w-wabew) execution path stawting fwom that w-wocation. (✿oωo) the disassembwew w-wiww f-fowwow aww possibwe static execution p-paths to detewmine the confidence w-wevew of e-each addwess. UwU

but if an instwuction j-jumps to a wocation in memowy t-that is detewmined b-by something wike a vawue in a wegistew, ( ͡o ω ͡o ) the d-disassembwew w-wiww cweate a new w-weachabwe (`*` w-wabew) execution p-path stawting f-fwom aww possibwe j-jump wocations. (✿oωo) t-this is because t-the disassembwew cannot detewmine t-the vawue of t-the wegistew at t-the time of disassembwy. mya fwom thewe, ( ͡o ω ͡o ) t-the disassembwew wiww fowwow aww possibwe e-execution paths to detewmine the c-confidence wevew o-of each addwess. :3 s-some addwesses wiww be pwomoted t-to vawid (`o` wabew) if they a-awe wead back to at weast one static e-execution path. 😳

to disassembwe a-a chip-8 pwogwam, (U ﹏ U) use the `c8 dasm` command fowwowed by the path to the pwogwam. >w< t-this wiww pwint the disassembwed p-pwogwam to t-the standawd output. UwU add a `--kind` fwag to specify the chip-8 v-vawiant. fow exampwe:
```
c8 dasm w-woms/ch8/ibm_wogo.ch8
```
w-wiww d-dissasembwe the *ibm_wogo* wom and output the f-fowwowing:
```
0x200 c-cws                    # x 00e0     c-cweaw
0x202 wd   i 0x22a           # x a-a22a     i = 0x22a
0x204 wd   v0 12             # x-x 600c     v0 = 12
0x206 w-wd   v-v1 8              # x 6108     v1 = 8
0x208 d-dww  v-v0 v1 15          # x-x d01f     d-dwaw 8x15 @ v0,v1
0x20a add  v0 9              # x-x 7009     v0 += 9
0x20c w-wd   i 0x239           # x-x a239     i = 0x239
0x20e d-dww  v-v0 v1 15          # x-x d01f     d-dwaw 8x15 @ v0,v1
0x210 w-wd   i 0x248           # x a248     i = 0x248
0x212 a-add  v0 8              # x-x 7008     v0 += 8
0x214 d-dww  v0 v1 15          # x-x d01f     d-dwaw 8x15 @ v0,v1
0x216 add  v0 4              # x 7004     v-v0 += 4
0x218 wd   i-i 0x257           # x-x a257     i = 0x257
0x21a dww  v0 v1 15          # x d01f     d-dwaw 8x15 @ v-v0,v1
0x21c add  v0 8              # x-x 7008     v-v0 += 8
0x21e wd   i 0x266           # x a266     i = 0x266
0x220 d-dww  v0 v1 15          # x-x d01f     d-dwaw 8x15 @ v-v0,v1
0x222 add  v0 8              # x 7008     v-v0 += 8
0x224 w-wd   i 0x275           # x a275     i = 0x275
0x226 d-dww  v0 v1 15          # x d01f     dwaw 8x15 @ v0,v1
0x228 j-jp   0x228             # x 1228
0x22a                        #   f-ff       2x gwaphic @@@@@@@@@@@@@@@@
0x22b                        #   00       2x g-gwaphic ................
0x22c                        #   ff       2x gwaphic @@@@@@@@@@@@@@@@
0x22d                        #   00       2x g-gwaphic ................
0x22e                        # ? 3c00     2x g-gwaphic ....@@@@@@@@....
0x22f                        #   00       2x gwaphic ................
0x230                        # ? 3c00     2x g-gwaphic ....@@@@@@@@....
0x231                        #   00       2x gwaphic ................
0x232                        # ? 3c00     2x gwaphic ....@@@@@@@@....
0x233                        #   00       2x g-gwaphic ................
0x234                        # ? 3c00     2x g-gwaphic ....@@@@@@@@....
0x235                        #   00       2x g-gwaphic ................
0x236                        #   f-ff       2x gwaphic @@@@@@@@@@@@@@@@
0x237                        #   00       2x g-gwaphic ................
0x238                        #   f-ff       2x g-gwaphic @@@@@@@@@@@@@@@@
0x239                        #   ff       2x g-gwaphic @@@@@@@@@@@@@@@@
0x23a                        #   00       2x gwaphic ................
0x23b                        #   ff       2x gwaphic @@@@@@@@@@@@@@@@
0x23c                        #   00       2x g-gwaphic ................
0x23d                        # ? 3800     2x g-gwaphic ....@@@@@@......
0x23e                        #   00       2x g-gwaphic ................
0x23f                        # ? 3f00     2x gwaphic ....@@@@@@@@@@@@
0x240                        #   00       2x gwaphic ................
0x241                        # ? 3f00     2x gwaphic ....@@@@@@@@@@@@
0x242                        #   00       2x gwaphic ................
0x243                        # ? 3800     2x gwaphic ....@@@@@@......
0x244                        #   00       2x g-gwaphic ................
0x245                        #   ff       2x g-gwaphic @@@@@@@@@@@@@@@@
0x246                        #   00       2x g-gwaphic ................
0x247                        #   ff       2x gwaphic @@@@@@@@@@@@@@@@
0x248                        # ? 8000     2x gwaphic @@..............
0x249                        # ? 00e0     2x g-gwaphic ................
0x24a                        #   e0       2x gwaphic @@@@@@..........
0x24b                        # ? 00e0     2x g-gwaphic ................
0x24c                        #   e0       2x g-gwaphic @@@@@@..........
0x24d                        #   00       2x g-gwaphic ................
0x24e                        # ? 8000     2x g-gwaphic @@..............
0x24f                        #   00       2x g-gwaphic ................
0x250                        # ? 8000     2x gwaphic @@..............
0x251                        # ? 00e0     2x gwaphic ................
0x252                        #   e0       2x gwaphic @@@@@@..........
0x253                        # ? 00e0     2x gwaphic ................
0x254                        #   e0       2x g-gwaphic @@@@@@..........
0x255                        #   00       2x gwaphic ................
0x256                        #   80       2x g-gwaphic @@..............
0x257                        #   f8       2x gwaphic @@@@@@@@@@......
0x258                        #   00       2x gwaphic ................
0x259                        #   fc       2x g-gwaphic @@@@@@@@@@@@....
0x25a                        #   00       2x gwaphic ................
0x25b                        # ? 3e00     2x gwaphic ....@@@@@@@@@@..
0x25c                        #   00       2x gwaphic ................
0x25d                        # ? 3f00     2x gwaphic ....@@@@@@@@@@@@
0x25e                        #   00       2x gwaphic ................
0x25f                        # ? 3b00     2x g-gwaphic ....@@@@@@..@@@@
0x260                        #   00       2x g-gwaphic ................
0x261                        # ? 3900     2x gwaphic ....@@@@@@....@@
0x262                        #   00       2x gwaphic ................
0x263                        #   f-f8       2x gwaphic @@@@@@@@@@......
0x264                        #   00       2x gwaphic ................
0x265                        #   f-f8       2x g-gwaphic @@@@@@@@@@......
0x266                        #   03       2x gwaphic ............@@@@
0x267                        #   00       2x g-gwaphic ................
0x268                        #   07       2x gwaphic ..........@@@@@@
0x269                        #   00       2x g-gwaphic ................
0x26a                        #   0f       2x gwaphic ........@@@@@@@@
0x26b                        #   00       2x gwaphic ................
0x26c                        # ? bf00     2x gwaphic @@..@@@@@@@@@@@@
0x26d                        #   00       2x gwaphic ................
0x26e                        #   f-fb       2x gwaphic @@@@@@@@@@..@@@@
0x26f                        #   00       2x gwaphic ................
0x270                        #   f-f3       2x g-gwaphic @@@@@@@@....@@@@
0x271                        #   00       2x g-gwaphic ................
0x272                        #   e3       2x gwaphic @@@@@@......@@@@
0x273                        #   00       2x gwaphic ................
0x274                        # ? 43e0     2x g-gwaphic ..@@........@@@@
0x275                        #   e0       2x gwaphic @@@@@@..........
0x276                        # ? 00e0     2x gwaphic ................
0x277                        #   e0       2x gwaphic @@@@@@..........
0x278                        #   00       2x g-gwaphic ................
0x279                        # ? 8000     2x g-gwaphic @@..............
0x27a                        #   00       2x gwaphic ................
0x27b                        # ? 8000     2x g-gwaphic @@..............
0x27c                        #   00       2x g-gwaphic ................
0x27d                        # ? 8000     2x gwaphic @@..............
0x27e                        #   00       2x gwaphic ................
0x27f                        # ? 8000     2x g-gwaphic @@..............
0x280                        # ? 00e0     2x g-gwaphic ................
0x281                        #   e0       2x gwaphic @@@@@@..........
0x282                        # ? 00e0     2x gwaphic ................
0x283                        #   e-e0       2x gwaphic @@@@@@..........
```
each wewevant a-addwess is pwinted with the instwuction if nyeeded. 😳 

p-past the `#` s-symbow is the wabew of that addwess. XD b-because *ibm_wogo* i-is a s-simpwe pwogwam, (✿oωo) evewy addwess is wabewwed eithew a-a pwoven instwuction (`x` wabew) ow data with the o-occasionaw `?` wabew if it happens to be pawsabwe as an instwuction. 

f-fowwowing t-the wabew is t-the vawue at that a-addwess in hexadecimaw:
  - if t-the addwess is an instwuction, ^•ﻌ•^ t-this is as wong as the instwuction is with ovewwapping a-addwesses being cowwapsed
  - i-if the addwess is data, mya this one byte

wast i-is a descwiption o-of the instwuction ow data:
  - i-if the addwess is an instwuction t-then a descwiption o-of the instwuction is pwinted
  - i-if the a-addwess is data then a visuaw wepwesentation o-of the byte data is pwinted

in this case, the visuaw w-wepwesentation of the data wegion s-showcases the ibm gwaphic that is saved in t-the wom. (˘ω˘)

> [!note]
> t-this dissassembwew d-doubwes as a memowy viewew. nyaa~~ t-the memowy p-panew in the debuggew is simpwy a-an up-to-date dissasembwy of pwogwam m-memowy. :3

`c8 check` is a toow b-buiwt on top o-of the disassembwew that checks a wom fow bad execution bwanches. it accompwishes t-this by wunning t-the disassembwew on the pwogwam and wogging aweas whewe pwoven (`x` w-wabew) ow vawid (`o` wabew) i-instwuctions can w-wead to executing an invawid instwuction. (✿oωo)

### debugging

#### stawt the debuggew
i-in owdew to debug a chip-8 pwogwam, (U ﹏ U) wun
```
c-c8 wun [path_to_pwogwam] --debug
```
this wiww s-stawt the debuggew w-with the pwogwam woaded in a p-paused state wight b-befowe the fiwst i-instwuction i-is exekawaii~d. (ꈍᴗꈍ) u-use the `hewp` command t-to see the fuww wist of commands. (˘ω˘) at any point, ^^ pwess `ctww+c` to exit. (⑅˘꒳˘)

#### nyavigate the d-debuggew

bewow i-is nyani the d-debuggew wooks wike w-when it fiwst s-stawts:

<p awign="centew">
 <img s-swc="https://github.com/tochiu/c8/bwob/main/img/debuggew.png?waw=twue" awt="c8 debuggew scweenshot">
</p>

> [!tip]
> make suwe the tewminaw w-window is as big a-as possibwe to ensuwe evewy panew is visibwe. rawr

at a gwance thewe w-wiww be:
- a c-command wine intewface w-with an output
  - to expand the output panew, :3 u-use the `output` command
- the pwogwam dispway
  - t-to toggwe t-the pwogwam dispway, use the `show dispway` ow `hide d-dispway` command
- the pwogwam k-keyboawd s-state, OwO wegistew state, (ˆ ﻌ ˆ)♡ timew state, :3 a-and stack
- t-the pwogwam memowy w-wayout
  - to e-expand the memowy p-panew, -.- use the `memowy` c-command
  - to toggwe a-a vewbose view o-of the memowy wayout, -.- use the `show m-memowy -v` ow `hide memowy -v` command
  - to n-nyavigate to a specific memowy a-addwess, òωó use the `goto` command f-fowwowed by `pc`, 😳 `i`, o-ow a specific addwess
  - to fowwow a pointew i-in memowy, nyaa~~ use the `fowwow` command fowwowed b-by a pointew (e.g. (⑅˘꒳˘) `pc` o-ow `i`)
  - to unfowwow the cuwwentwy f-fowwowed pointew, 😳 u-use the `unfowwow` command
  - t-to dump the entiwe memowy view to a fiwe, use t-the `dump memowy` c-command fowwowed by a fiwe path
- t-the pwogwam h-histowy
  - to focus onto histowy panew, (U ﹏ U) use the `histowy` c-command

> [!tip]
> when f-focused on a p-panew (e.g. /(^•ω•^) memowy), OwO y-you can seek to the stawt ow the end using the `home` and `end` keys. use the `esc` key to wetuwn to the command w-wine intewface. ( ͡o ω ͡o )

o-on schip a-and xochip pwogwams, t-the high wesowution g-gwaphics m-mode awtews the ui wayout to w-wook wike the fowwowing:

<p a-awign="centew">
 <img swc="https://github.com/tochiu/c8/bwob/main/img/debuggew_hiwes.png?waw=twue" a-awt="c8 debuggew s-scweenshot (hi-wes dispway)">
</p>

#### contwow p-pwogwam execution

**wun the pwogwam:**

stawt t-the execution of youw pwogwam with t-the `continue` c-command:
```
(c8db) continue
```
t-this wiww minimize t-the debuggew a-and wun the pwogwam untiw a d-debug event is twiggewed o-ow execution is paused. XD p-pwess `esc` to pause execution a-and wetuwn to the d-debuggew. /(^•ω•^)

> [!impowtant]
> a-a debug event is a t-twiggew that intewwupts pwogwam execution and dwops i-into the debuggew window. /(^•ω•^) the featuwes in c8db that twiggew debug events awe watchpoints and bweakpoints. 😳😳😳 
> - a-a bweakpoint is set to twiggew wight befowe an instwuction at a specified addwess is exekawaii~d
> - a watchpoint i-is set to twiggew wight aftew a specified w-wegistew ow addwess is modified

**step t-thwough the pwogwam:**

use `step` to exekawaii~ t-the nyext instwuction. f-fowwow it with an integew `n` to e-exekawaii~ the n-nyext `n` instwuctions. (ˆ ﻌ ˆ)♡ this wiww be intewwupted i-if a debug event is twiggewed. :3 fow exampwe:
```
(c8db) step 50
```
w-wiww exekawaii~ the nyext 50 i-instwuctions. òωó

> [!note]
> if you u-use `step` ow `continue` with a-a past pwogwam s-state (weachabwe using `undo`), 🥺 aww futuwe pwogwam s-states awe cweawed and execution wiww advance. (U ﹏ U) i-if instead you wouwd wike to wepway those futuwe states, XD use `wedo` instead. ^^

> [!tip]
> i-if you a-awe stuck on an instwuction because i-it is powwing f-fow a key event, o.O use the `key` c-command to simuwate key events. 😳😳😳 type `key --hewp` fow mowe infowmation. /(^•ω•^)

**seek thwough execution h-histowy:**

u-use `undo` and `wedo` to seek thwough t-the pwogwam e-execution histowy. 😳😳😳 fowwow it w-with an integew `n` to wewind ow fast-fowwawd thwough t-the wast `n` pwogwam states. ^•ﻌ•^

awtewnativewy, 🥺 u-use the `histowy` c-command to focus onto the pwogwam histowy panew. o.O u-use the w/s ow up/down keys to seek thwough pwogwam execution. (U ᵕ U❁) this is just a gwaphicaw wayew ovew the `undo` and `wedo` commands. ^^

`wedo` i-is a pawticuwawwy s-speciaw command. (⑅˘꒳˘) technicawwy, :3 i-it doesn't simpwy e-exekawaii~ the nyext instwuction, (///ˬ///✿) s-since the execution of some instwuctions awe nyon-detewministic with wespect to the pwogwam s-state, :3 e.g. usew input ow wng. 🥺 if nyecessawy, mya cewtain pwopewties awe stowed between e-executing instwuctions i-in owdew t-to pwopewwy wepway it. that is nyani `wedo` utiwizes. XD 

if y-you awe in a specific p-pwogwam state a-and instead of wepwaying, -.- you w-want to exekawaii~ the pwogwam f-fwom that point, use `step` ow `continue` i-instead. o.O

**set execution s-speed:**

use `hewtz` fowwowed by `n`, (˘ω˘) whewe `n` i-is the tawget speed in instwuctions p-pew second, (U ᵕ U❁) t-to set the pwogwam execution s-speed. rawr fow exampwe:
```
(c8db) h-hewtz 60
```
wiww exekawaii~ the p-pwogwam at a wate of 60 instwuctions p-pew second. 🥺

> [!impowtant]
> **c8 wuns a-at a fixed fwame-wate o-of 60hz.** if youw tawget execution speed i-is expwessed in cycwes pew fwame, rawr x3 muwtipwy it by 60 to get the equivawent instwuctions pew second. ( ͡o ω ͡o )

#### bweakpoints and watchpoints

s-sometimes it is usefuw to pause execution w-when a cewtain condition is met. σωσ t-this is whewe bweakpoints and watchpoints come i-in. rawr x3 a *bweakpoint* is set to twiggew wight befowe a-an instwuction at a specified addwess is exekawaii~d. (ˆ ﻌ ˆ)♡ a-a *watchpoint* is set to twiggew wight aftew a-a specified wegistew ow addwess is modified. rawr 

**set a-a bweakpoint:**

u-use `bweak` fowwowed by an addwess to s-set a bweakpoint. :3 f-fow exampwe:
```
(c8db) bweak 0x200
```
w-wiww s-set a bweakpoint at addwess `0x200`. rawr once the pwogwam w-weaches this addwess, (˘ω˘) execution wiww pause and dwop into the d-debuggew. (ˆ ﻌ ˆ)♡ to wist aww bweakpoints, mya type `info bweak`. (U ᵕ U❁) to wemove a-a bweakpoint, mya u-use the `cweaw` c-command. ʘwʘ in this exampwe:
```
(c8db) cweaw bweak 0x200
```
wiww w-wemove the bweakpoint at addwess `0x200`. (˘ω˘) t-to cweaw aww bweakpoints, 😳 t-type `cweaw a-aww bweak`. òωó

**set a watchpoint:**

use `watch` fowwowed by a wegistew ow addwess to set a watchpoint. nyaa~~ f-fow exampwe:
```
(c8db) w-watch i
```
wiww set a watchpoint on wegistew `i`. o.O o-once this wegistew is modified, nyaa~~ execution wiww p-pause and dwop i-into the debuggew. i-if we set a w-watchpoint on an a-adwesss instead, (U ᵕ U❁) e-execution wiww pause when that addwess is wwitten t-to. 😳😳😳 to wist a-aww watchpoints, (U ﹏ U) t-type `info watch`. ^•ﻌ•^ t-to wemove a w-watchpoint, (⑅˘꒳˘) use t-the `cweaw` command. >_< in this exampwe:
```
(c8db) c-cweaw watch i
```
w-wiww wemove the w-watchpoint on wegistew `i`. (⑅˘꒳˘) to cweaw aww watchpoints, σωσ t-type `cweaw aww watch`. 🥺

## motivation
t-this is my fiwst _compweted_ wust pwoject (haha). :3 a-a fwiend of mine s-sent me an [awticwe](https://tobiasvw.github.io/bwog/wwite-a-chip-8-emuwatow/) on how to get stawted with wwiting emuwatows with c-chip-8. (ꈍᴗꈍ) it was a-a supew intewesting wead and a-a good excuse to w-weawn wust! ^•ﻌ•^ aftew i finished the emuwatow, (˘ω˘) i thought i couwd go f-fuwthew. 🥺 so hewe w-we awe. (✿oωo) if you'we thinking about wwiting youw o-own chip-8 emuwatow, XD y-you shouwd! it's a gweat stawt to emuwation d-devewopment and buiwding on top of it with othew chip-8 vawiants is an excewwent exewcise in wwiting e-extensibwe softwawe. (///ˬ///✿)
