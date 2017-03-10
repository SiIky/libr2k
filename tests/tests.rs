extern crate libr2k;

use libr2k::dict::{Dict, KanaConvertionTable};

#[cfg(test)]
#[test]
fn test1() {
    let d: Dict = Dict::dnew();
    let input: String = "Kimi ni kirawareta kimi no Chinmoku ga kikoeta\nKimi no me no mae ni iru no ni Tooku \
         kara kikoeta\nHasshingen wo sagashitara Tadoritsuita mizutamari\nKore ga hito no kokoro \
         nara Fukasa nado wakaranai\nYobareta no ga Boku demo boku ja nai to shitemo\nDou demo ii \
         koto darou Mondai wa betsu ni arun da\nIki wa motsu darou ka Fukai kokoro no soko \
         made\nKimi ga shizumeta kimi wo Mitsukeru made moguru tsumori sa\nKurushisa to hirei \
         shite Bokura wa chikazukeru\nFutatabi kokyuu wo suru toki wa Kimi to issho ni\nBoku mo \
         mata onnaji you ni Chinmoku wo kikareta\nKimi mo mata onnaji you ni Tobikonde kureru \
         nara\nKuchizuke wo azukeaou Nakusazu ni motte ikou\nKimi ni kirawareta kimi e Kawari ni \
         todokeru yo\nDaremo ga chigau ikimono Tanindoushi dakara\nSabishisa wo shitta toki wa \
         Nukumori ni kizukerun da\nYuuki wa aru darou ka Ichido kokoro nozoitara\nKimi ga \
         kakushita itami Hitotsu nokorazu shitte shimau yo\nKizutsukeru kawari ni Onaji dake \
         kizutsukou\nWakachiaeru mon ja nai no nara Nibai areba ii\nKowai no sa Boku mo kimi \
         mo\nJibun wo miru no mo miseru no mo Arui wa dareka wo nozoku no mo\nDemo Seiippai \
         okutte ita Shizumeta jibun kara\nInoru you na meedee\nHibiku kyuunan shingou Fukai \
         kokoro no katasumi\nKonna tokoro ni ita no Soba ni oide Nigenakute ii yo\nFureta \
         hasshingen ni Todokeru yo Kuchizuke\nKimi kara azukatte kitan da yo\nYuuki wa aru darou \
         ka Ichido te wo tsunaidara\nHanasanai mama soto made Tsurete iku yo Shinjite ii yo\nIki \
         wa motsu darou ka Mabushii kokoro no soto made\nFutatabi kokyuu wo suru toki wa Kimi to \
         issho ni\nKanjani8 - Yorimichi\nAshita dou ni ka naru tte Amaete bakka irun ja\nKekkyoku \
         nanimo mitsukarazu ni Mata Tameiki\nSonnan rashiku nai tte Iwarete bakka irun ja\nDame \
         ni naru...sonna ochi!!\n Kanaetai yume oikakete\nKoko made kitan da wow\nKuyashi namida \
         nagasetara\nChotto Raku ni nan darou? Yeah!!\n Yorimichi shita tte ii Isoganakutemo \
         ii\nYaritai koto ga aru no wa suteki de\nDare ni mo nai chikara motterun da\nNaita tte \
         ii Kedo Mou ichido kao agete\nHora Nayanderu hima nante nai no sa\nKoronde kizutsuitta \
         tte Hekonde bakka irun ja\nNandomo onaji koto kurikaeshi Mata Tameiki\nTsurai no ni iji \
         hatte Muri shite bakka irun ja\nKakkowarui Sore ga ochi!!\n Yama wo koe tani wo \
         koete\nKirihiraitekun da wow\nChansu ga soko ni aru nara\nMotto Gattsuken daro? Yeah!!\n \
         Mayowazu yareba ii Shinjite susumeba ii\nYaritogeru sono yuuki ga daiji de\nItsuka \
         dekkai yume tsukamitorun da\nKoronda tte ii Ima Me no mae no kabe fukitobashite\nMotto \
         Takaku ue mezasun da\nKimi wa itsu datte hitori ja nai no sa\nGanbatteru sugata Zutto \
         miteru kara\nHashitte ike yo\nYorimichi shita tte ii Isoganakutemo ii\nYaritai koto ga \
         aru no wa suteki de\nDare ni mo nai chikara motterun da\nNaita tte ii Kedo Mou ichido \
         kao agete\nHora Nayanderu hima nante nai no sa\nSaa Hashiridase Mada hajimatta bakari \
         sa\nKanjani8 - Ano kotoba ni\nDonna ni ganbattemo kotae ga\nMiete konai toki ga \
         aru\nSaki ga mienai genjitsu ni\nFumitodomatte ita\nKizutsuitari Miushinattari\nShidai \
         ni karete yuku\nBoku no daichi ni Kimi no kotoba\nMichishirube ni naru\nHito wa toki ni \
         makesou ni narun dakedo\nHoka no dareka ni Senaka wo osaretari\nFushigi da yo ne Ano \
         toki no kotoba ni\nIma mo boku wa shigamitsuite Ikitetari shiteru\nArigatou.\n Fuan na \
         yoru ga aru darou\nKokorobosokute nakitai yoru ga\nKono mama kiete shimaeba ii\nSou \
         omoeru toki ga\nKodoku toiu Ame no naka wo\nHashiritsuzuketeta toki\nKimi ga sashikakete \
         kureta\nKasa ni hairikomu\nKono sekai no donna chiisana koto sae mo\nHitorikiri ja \
         Nashienain dakara\nHito ga hito to tsunagaru kizuna no you ni\nDareka ga boku wo sasaete \
         iru Sonna ki ga shiteru\nHito wa toki ni makesou ni narun dakedo\nHoka no dareka ni \
         Senaka wo osaretari\nFushigi da yo ne Ano toki no kotoba wa\nIma mo boku wo sasaete iru \
         Kore kara mo zutto\nArigatou.\n \nanjani8 - Ittsu Mai Souru\nSoo demo nai daiichi \
         inshou\nMeshikai demo ki ga kiku wake jaa nai\nOH beibii Na no ni kimi Sakana no \
         tabekata ga kirei\nIya Maitta Soko wa tsubo\nJasutomiito Koi no suicchi wa on\nOre yowai \
         no Sooyuu no\nShuujitsu Mou kimi ni muchuu OH beibii\nKimi no tame ni ikiyou ka\nHoreta \
         mon dakara shikata nai\nTatoe ore ga nibante demo\nRararararara Massorya shou ga \
         nai\nNijikan no machibouke\nSore gurai wa yoshi to shiyou\nTodomesasu dotakyan \
         mo\nShogetari shinai-i Oikakeru Ittsu Mai Souru\nUchiwa de wa saikai datta kimi\nMa-ima \
         mo sore hodo kawaii tte wake ja nai\nOH beibii Sara ni kimi Yasashisa tte iu Kanjou mo \
         usuku nai?\n Sono kuse ni dekiru no wa\nSuupaa no fukuro shimattoku yoo na toko\nSooyuu \
         toko tsubo na no sa\nShuujitsu Aa kimi ni muchuu OH beibii\nKimi no tame ni ikitai \
         na\nHoreta mon dakara shikata nai\nArienai wagamama mo\nRararararara Massorya shou ga \
         nai\nShinya niji no omukae kooru\nGutto taete yoshi to shiyou\nHoreta ga make da \
         nante\nMyou ni nattoku no Monku gatta Ittsu Mai Souru\nGatta gatta gatta\nMonku monku \
         monku\nGatta gatta gatta Ittsu Mai Souru\nOo Mushinkei na kotoba wo\nAa Kikanjuu no you \
         ni\nSaihou ga tokui dattari suru ka-ra\nTsubo yanen\nKimi wo omoidasanai\nSonna yoru wa \
         iranai\nNandakanda de Yappa suki\nShuujitsu Kimi de OH Ittsu Mai Souru\nKimi no tame ni \
         ikiyou ka\nHoreta mon dakara shikata nai\nTatoe ore ga nibante demo\nRararararara \
         Massorya shou ga nai\nNijikan no machibouke\nSore gurai wa yoshi to shiyou\nTodomesasu \
         dotakyan mo\nShogetari shinai-i Oikakeru Ittsu Mai Souru\nTOKIO - Hitsuyou to omowareru \
         kasho ni piriodo wo ute\nMona \nTom It's no money cow (Itsu no ma ni ka)\nDave I got it! \
         AHH! Cool tail!! (Ai ga itakute)\nTom It's no money cow (Itsu no ma ni ka)\nDave You got \
         it! AHH! Cool tail!! (Yogaritakute)\nMona\nTranslate following words into English\nand \
         them put your favourite color into [D]\nTOKIO - Yatto aeta\nKonna ni kanjitan da yo Kimi \
         dato wakattan da yo\nKokoro ni tobikonde kita Mabushii deai sa\nTotsuzen Tegami kaitari \
         Futari de tokei Kaetari\nWaratte Kata wo narabete Arukihajimeru\nHoshi ni negai kaketa \
         wake ja nai kedo Boku wa\nZutto kimi no koto wo sagashite ita yo Hitori de koko made \
         kita yo\nYatto aeta bokura dakara\nAsu mo kitto kono te wo tsunagou\nTsurai toki ga aru \
         to shitemo\nKono mune ni umareta saikou no dekigoto\nBoku ni wa kimi ga iru kara\nMachi \
         no kaze ga Itsuka kawaru to shitemo Boku wa\nKitto kimi no namida nugutte itai Ashita wo \
         utatte itai\nYatto aeta bokura dakara\nFukaku zutto Dakishimeaun da\nTsurai toki ga aru \
         to shitemo\nKono mune ni umareta saikou no dekigoto\nKimi ni wa boku ga iru kara\nKonna \
         ni kanjitan da yo Kimi dato wakattan da yo\nKokoro ni tobikonde kita Mabushii deai \
         sa\nFutari de tooku ittari Toki ni wa nayande mitari\nWaratte Kata wo narabete \
         Arukihajimete yukou\nTOKIO - Watasenai enjeru\nKeep on Rollin' Keep on Goin'\nKimi wo \
         tsurete iku yo\nWakatte Ita no sa\nKimi wa Kimi wa Saigo no tenshi\nYume nante Kotoba \
         sae\nUbawareta Jidai ni\nCarry on! Carry on!\n Ai wo shinjitai\nSetsunasa ga Kasoku \
         shite\nKono mune ga Kowareta\nHurry up! Hurry up!\n Subete Kimi no sei\nKamen no you na \
         Hitogomi no naka kara\nAsu wo sagashite Bokura wa hashiridasu\nBaby, I want you!\n Keep \
         on Rollin' Keep on Goin'\nTatakitsuzukeru no sa\nSubete no tobira wo Mune no itami \
         wo\nKeep on Rollin' Keep on Lovin'\nKimi wo tsurete iku yo\nWakatte Ita no sa\nKimi wa \
         Kimi wa Saigo no tenshi\nDensetsu ni Naru you na\nKoi wo suru Futari sa\nKiss me now! \
         Kiss me now!\n Hadaka no kokoro de\nNamida ya uso ni Yogosarete ittemo\nHyakunen kienai \
         Kisu wo boku wa hoshii\nBaby, I love you!\n Keep on Rollin' Keep on Goin'\nDakara ima \
         shika nai\nKizutsuku Koto dake Erandemo ii\nKeep on Rollin' Keep on Lovin'\nHidoku \
         dakishimetai\nKimi shika Nai no sa\nToki ga Kureta Saigo no tenshi\nKeep on Rollin' Keep \
         on Goin'\nHashiritsuzukeru no sa\nTsukanda Kono te wo Hanashi wa shinai\nKeep on Rollin' \
         Keep on Lovin'\nKimi wo tsurete iku yo\nWakatte Ita no sa\nKimi wa Kimi wa Saigo no \
         tenshi\nTOKIO - Kono yoru wo koete\nKimi no kanashimi wo\nSubete ubaitai\nSuki nan da \
         Sore dake sa Hold me tight Hold me tight\nKono yoru wo koete\nNakigoe de kireta Denwa wo \
         nagedashi\nMado kara tobiori Boku wa kakedashita\nNemutta Machinami Donna ni \
         isoidemo\nOmoi no Hayasa ni Ashi ga oitsukanakute\nWoo- Kokoro dake ja \
         Tsutawaranai\nKotoba ga jama nan da Loneliness (Loneliness)\nWoo- Sore demo ii Soba ni \
         iku yo\nNanimo dekinakutemo Runaway (Runaway)\nNamida fuite Hohoendara\nHanasanai \
         kara\nIt's all right Kimi no kanashimi wo\nIt's all right Subete ubaitai\nSuki nan da \
         Sore dake sa Hold me tight Hold me tight\nIt's all right Tarinai mono nado\nIt's all \
         right Ikutsumo aru kedo\nDakishimete Itai no sa Baby, it's you Yes, it's you\nDenwa \
         bokkusu de Hiza wo kakaeteru\nKimi wo omou tabi Mune ga itamu no sa\nTe ni irerarenai \
         Yume no kakera yori\nKimi wo nakaseta Sore ga kuyashikatta\nWoo- Kaisatsuguchi \
         Hanarerarezu\nToki wo tometaku naru Every night (Every night)\nWoo- Futaribun no \
         Samishisa demo\nKasaneaeru no nara Fly away (Fly away)\nItsuka kitto sore bakari \
         wo\nKurikaeshiteru\nIt's all right Tsubasa wo kurenai\nIt's all right Sekai no \
         katasumi\nMitsumetete Hoshii no sa Hold me tight Hold me tight\nIt's all right Tarinai \
         mono nado\nIt's all right Ikutsumo aru kedo\nDakishimete Itai no sa Baby, it's you Yes, \
         it's you\nKimi wo shinjiteru\nBoku wo shinjitai\nKono yoru wo koete\nTOKIO - Aitsu ni \
         yoroshiku\nHassha no beru ga naru yo\nMou ikanakucha\nShinyuu no omae dakara\nDengon wo \
         takusu yo\nAitsu ni itte kure yo\nDaisuki datta\nSayonara mo ienai mama\nTooi machi ni \
         iku kedo\nOmoide wo Thank you, woo my girl\nTsurai kedo Good-by Honto no sotsugyou \
         sa\nHitori ni shinai tte\nYakusoku shita no ni ne\nHonki de oikaketain da\nYume made \
         hashiru yo\nIkinari dakishimete\nHageshii kisu no ame\nFuraseta ano hi no riyuu \
         ni\nKizuite naitemo\nAitsu ni yoroshiku\nDokoka de koe ga suru yo\nMune no \
         katasumi\nHanaretemo umaku yareru\nSonna ki mo suru kedo\nAitsu ga kawaisou sa\nMou \
         kimetan da\nIi yatsu ga mitsukaru hazu\nMotto kirei ni nareru\nKokoro kara Thank \
         you,\nwoo my love\nFurimukazu Good-by\nManatsu no sotsugyou sa\nFutari no machinami \
         ga\nTooku ni kieteku yo\nSekai de ichiban suteki na\nShiawase inoru yo\nIchido wa dare \
         datte\nMukaeru tabidachi sa\nKosame ni nureteru hodou wo\nNamida de kaketemo\nAitsu ni \
         yoroshiku\nTOKIO - Girl\nGirl, usotsuki na kimi Ima mo\nUhm- Oboete iru yo Zutto\nGirl, \
         sayonara iezu Kimi wa\nUhm- Boku wo miagete Naita ne\nYume wo kazoete \
         Dakishimeatta\nItsumo, itsu demo\nKimi wo mitsumeteta\nBye Bye Tomadoinagara (Baby, Bye \
         Bye Uh-)\nTada, kata wo daite ita\nBye Bye Itsuka wa kimi ga\nYuku no wo shitteta \
         Girl....\n (I was in love with you my girl)\n(I was in love)\nGirl, saigo ni kureta Kiss \
         ga\nUhm- Mune ni sasatte Nukenai\nGirl, ima demo kimi wa Kitto\nUhm- Boku ga suki da, to \
         Iu yo ne\nOtona ni natta Kimi no hohoemi\nKanashii kurai\nBoku wa kizuiteta\nBye Bye \
         Wasurete ii yo (Baby, Bye Bye Uh-)\nMou, todokanai Kimi wa\nBye Bye Tomodachi ni sae \
         (Baby, Bye Bye)\nMou, narenai yo Kitto\nBye Bye Girl Kagayaki dake wo\nIma demo \
         Nokoshite Girl\n(I was in love with you my girl)\n(I was in love)"
            .to_string();

    let output: String = libr2k::to_kana(&d, &input);
    assert!(input != output);
}

#[test]
fn test2() {
    let d: Dict = Dict::dnew();
    let input: String = "kan'i kani".to_string();

    let kana: String = libr2k::to_kana(&d, &input);
    let hira: String = libr2k::to_hiragana(&d, &input);

    let should_be: String = "かん'いかに".to_string();

    assert_eq!(kana, should_be);
    assert_eq!(hira, should_be);
}

#[test]
fn test3() {
    let d: Dict = Dict::dnew();

    let input: Vec<String> = (vec!["ka", "shi", "i", "sa", "さ"])
        .iter()
        .map(|s| s.to_string())
        .map(|s| libr2k::convert_syllable(&d, &s))
        .collect();

    let should_be: Vec<String> =
        (vec!["か", "し", "い", "さ", "さ"]).iter().map(|s| s.to_string()).collect();

    assert_eq!(input, should_be);
}
