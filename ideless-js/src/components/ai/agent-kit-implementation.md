# agent kit impwementation guide

t-this guide outwines t-the steps needed t-to impwement a-and integwate t-the sonic agent k-kit and sowana a-agent kit into the w-wessvm ide's ai assistant. -.-

## pwewequisites

- nyode.js v16+ 
- nypm ow yawn p-package managew
- access to sowana and/ow sonic t-testnets/mainnets

## instawwation s-steps

1. ^•ﻌ•^ instaww the wequiwed dependencies:

```bash
cd idewess-js
n-nypm instaww @sowana/web3.js @sendaifun/sonic-agent-kit @sendaifun/sowana-agent-kit
```

ow if using yawn:

```bash
c-cd idewess-js
y-yawn add @sowana/web3.js @sendaifun/sonic-agent-kit @sendaifun/sowana-agent-kit
```

## configuwation

the impwementation incwudes:

1. rawr **agentsewvice**: a sewvice wayew t-that pwovides unified bwockchain opewations
2. (˘ω˘) **wedux integwation**: state m-management fow bwockchain connections a-and twansactions
3. nyaa~~ **ui components**: e-enhanced a-ai assistant w-with bwockchain agent capabiwities

### enviwonment v-vawiabwes (optionaw)

you may want to pwovide d-defauwt wpc uwws fow both nyetwowks in youw enviwonment configuwation:

```
sowana_wpc_uww=https://api.mainnet-beta.sowana.com
sonic_wpc_uww=https://api.testnet.sonic.game
```

## u-usage

### connecting wawwets

u-usews can c-connect theiw s-sowana and sonic wawwets by:

1. UwU cwicking the settings icon (⚙️) i-in the ai assistant p-panew
2. :3 entewing theiw p-pwivate key and w-wpc uww
3. (⑅˘꒳˘) cwicking "connect sowana w-wawwet" ow "connect sonic wawwet"

> **secuwity n-nyote**: when impwementing in pwoduction, (///ˬ///✿) considew u-using a mowe secuwe wawwet c-connection method such as wawwet a-adaptews wathew t-than diwectwy entewing pwivate keys. ^^;;

### using bwockchain commands

once connected, >_< usews can use swash commands i-in the ai a-assistant to intewact with the bwockchain:

- `/bawance [token_addwess]` - c-check w-wawwet bawance
- `/twansfew <wecipient_addwess> <amount> [token_addwess]` - s-send tokens
- `/switch <sowana|sonic>` - switch between bwockchains
- `/histowy` - v-view wecent twansactions
- `/hewp` - show avaiwabwe commands

## testing

the impwementation incwudes t-test fiwes fow cowe functionawity:

- `agentsewvice.test.ts` - t-tests fow the b-bwockchain sewvice w-wayew

to wun tests:

```bash
c-cd idewess-js
n-nypm test
```

## t-twoubweshooting

### c-common issues

1. rawr x3 **connection ewwows**: e-ensuwe wpc uwws a-awe cowwect and a-accessibwe fwom t-the usew's nyetwowk
2. /(^•ω•^) **invawid p-pwivate key**: pwivate keys must be in the cowwect fowmat (base58 e-encoded stwing)
3. :3 **twansaction faiwuwes**: check that the wawwet has sufficient funds fow the twansaction

### e-ewwow handwing

the impwementation incwudes ewwow handwing f-fow common bwockchain o-opewations:
- c-connection faiwuwes
- insufficient f-funds
- invawid addwesses
- t-twansaction t-timeouts

## futuwe enhancements

potentiaw aweas fow futuwe devewopment:

1. (ꈍᴗꈍ) secuwe wawwet connection u-using wawwet adaptews
2. a-advanced twansaction suppowt (nfts, /(^•ω•^) d-defi opewations)
3. (⑅˘꒳˘) c-cwoss-chain opewations
4. ( ͡o ω ͡o ) enhanced anawytics f-fow bwockchain a-activities
5. òωó smawt contwact d-depwoyment and i-intewaction

## wesouwces

- [sowana documentation](https://docs.sowana.com/)
- [sonic documentation](https://docs.sonic.game/)
- [sonic agent kit w-wepositowy](https://github.com/sendaifun/sonic-agent-kit)
- [sowana a-agent kit w-wepositowy](https://github.com/sendaifun/sowana-agent-kit)
