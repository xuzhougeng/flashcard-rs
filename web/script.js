// Romaji to Japanese Dictionary
const romajiDict = {
    'a': { romaji: 'a', hiragana: 'あ', katakana: 'ア', examples: ['愛(あい/ai) - love', '青い(あおい/aoi) - blue', '秋(あき/aki) - autumn'] },
    'i': { romaji: 'i', hiragana: 'い', katakana: 'イ', examples: ['家(いえ/ie) - house', '犬(いぬ/inu) - dog', '色(いろ/iro) - color'] },
    'u': { romaji: 'u', hiragana: 'う', katakana: 'ウ', examples: ['海(うみ/umi) - sea', '牛(うし/ushi) - cow', '歌(うた/uta) - song'] },
    'e': { romaji: 'e', hiragana: 'え', katakana: 'エ', examples: ['駅(えき/eki) - station', '絵(え/e) - picture', '英語(えいご/eigo) - English'] },
    'o': { romaji: 'o', hiragana: 'お', katakana: 'オ', examples: ['音(おと/oto) - sound', '男(おとこ/otoko) - man', '女(おんな/onna) - woman'] },
    'ka': { romaji: 'ka', hiragana: 'か', katakana: 'カ', examples: ['顔(かお/kao) - face', '火(ひ/hi) - fire', '川(かわ/kawa) - river'] },
    'ki': { romaji: 'ki', hiragana: 'き', katakana: 'キ', examples: ['木(き/ki) - tree', '黄色(きいろ/kiiro) - yellow', '聞く(きく/kiku) - listen'] },
    'ku': { romaji: 'ku', hiragana: 'く', katakana: 'ク', examples: ['口(くち/kuchi) - mouth', '靴(くつ/kutsu) - shoes', '雲(くも/kumo) - cloud'] },
    'ke': { romaji: 'ke', hiragana: 'け', katakana: 'ケ', examples: ['毛(け/ke) - hair', '今日(きょう/kyou) - today', '犬(けん/ken) - dog'] },
    'ko': { romaji: 'ko', hiragana: 'こ', katakana: 'コ', examples: ['子供(こども/kodomo) - child', '心(こころ/kokoro) - heart', '声(こえ/koe) - voice'] },
    'sa': { romaji: 'sa', hiragana: 'さ', katakana: 'サ', examples: ['魚(さかな/sakana) - fish', '桜(さくら/sakura) - cherry blossom', '寒い(さむい/samui) - cold'] },
    'shi': { romaji: 'shi', hiragana: 'し', katakana: 'シ', examples: ['白(しろ/shiro) - white', '知る(しる/shiru) - know', '死(し/shi) - death'] },
    'su': { romaji: 'su', hiragana: 'す', katakana: 'ス', examples: ['好き(すき/suki) - like', '少し(すこし/sukoshi) - a little', '住む(すむ/sumu) - live'] },
    'se': { romaji: 'se', hiragana: 'せ', katakana: 'セ', examples: ['世界(せかい/sekai) - world', '先生(せんせい/sensei) - teacher', '背(せ/se) - back'] },
    'so': { romaji: 'so', hiragana: 'そ', katakana: 'ソ', examples: ['空(そら/sora) - sky', '外(そと/soto) - outside', 'そう(sou) - so'] },
    'ta': { romaji: 'ta', hiragana: 'た', katakana: 'タ', examples: ['高い(たかい/takai) - high', '食べる(たべる/taberu) - eat', '誰(だれ/dare) - who'] },
    'chi': { romaji: 'chi', hiragana: 'ち', katakana: 'チ', examples: ['父(ちち/chichi) - father', '小さい(ちいさい/chiisai) - small', '血(ち/chi) - blood'] },
    'tsu': { romaji: 'tsu', hiragana: 'つ', katakana: 'ツ', examples: ['月(つき/tsuki) - moon', '机(つくえ/tsukue) - desk', '作る(つくる/tsukuru) - make'] },
    'te': { romaji: 'te', hiragana: 'て', katakana: 'テ', examples: ['手(て/te) - hand', '天気(てんき/tenki) - weather', '寺(てら/tera) - temple'] },
    'to': { romaji: 'to', hiragana: 'と', katakana: 'ト', examples: ['友達(ともだち/tomodachi) - friend', '年(とし/toshi) - year', '時(とき/toki) - time'] },
    'na': { romaji: 'na', hiragana: 'な', katakana: 'ナ', examples: ['名前(なまえ/namae) - name', '夏(なつ/natsu) - summer', '何(なに/nani) - what'] },
    'ni': { romaji: 'ni', hiragana: 'に', katakana: 'ニ', examples: ['肉(にく/niku) - meat', '日本(にほん/nihon) - Japan', '庭(にわ/niwa) - garden'] },
    'nu': { romaji: 'nu', hiragana: 'ぬ', katakana: 'ヌ', examples: ['布(ぬの/nuno) - cloth', '脱ぐ(ぬぐ/nugu) - take off', '盗む(ぬすむ/nusumu) - steal'] },
    'ne': { romaji: 'ne', hiragana: 'ね', katakana: 'ネ', examples: ['猫(ねこ/neko) - cat', '寝る(ねる/neru) - sleep', '値段(ねだん/nedan) - price'] },
    'no': { romaji: 'no', hiragana: 'の', katakana: 'ノ', examples: ['野(の/no) - field', '飲む(のむ/nomu) - drink', '乗る(のる/noru) - ride'] },
    'ha': { romaji: 'ha', hiragana: 'は', katakana: 'ハ', examples: ['花(はな/hana) - flower', '母(はは/haha) - mother', '春(はる/haru) - spring'] },
    'hi': { romaji: 'hi', hiragana: 'ひ', katakana: 'ヒ', examples: ['火(ひ/hi) - fire', '日(ひ/hi) - day/sun', '人(ひと/hito) - person'] },
    'fu': { romaji: 'fu', hiragana: 'ふ', katakana: 'フ', examples: ['富士(ふじ/fuji) - Mt. Fuji', '冬(ふゆ/fuyu) - winter', '二つ(ふたつ/futatsu) - two'] },
    'he': { romaji: 'he', hiragana: 'へ', katakana: 'ヘ', examples: ['部屋(へや/heya) - room', '返事(へんじ/henji) - reply', '減る(へる/heru) - decrease'] },
    'ho': { romaji: 'ho', hiragana: 'ほ', katakana: 'ホ', examples: ['本(ほん/hon) - book', '星(ほし/hoshi) - star', '欲しい(ほしい/hoshii) - want'] },
    'ma': { romaji: 'ma', hiragana: 'ま', katakana: 'マ', examples: ['町(まち/machi) - town', '窓(まど/mado) - window', '毎日(まいにち/mainichi) - everyday'] },
    'mi': { romaji: 'mi', hiragana: 'み', katakana: 'ミ', examples: ['水(みず/mizu) - water', '見る(みる/miru) - see', '耳(みみ/mimi) - ear'] },
    'mu': { romaji: 'mu', hiragana: 'む', katakana: 'ム', examples: ['村(むら/mura) - village', '胸(むね/mune) - chest', '六つ(むっつ/muttsu) - six'] },
    'me': { romaji: 'me', hiragana: 'め', katakana: 'メ', examples: ['目(め/me) - eye', '女(め/me) - woman', '飯(めし/meshi) - meal'] },
    'mo': { romaji: 'mo', hiragana: 'も', katakana: 'モ', examples: ['森(もり/mori) - forest', '桃(もも/momo) - peach', '文字(もじ/moji) - letter'] },
    'ya': { romaji: 'ya', hiragana: 'や', katakana: 'ヤ', examples: ['山(やま/yama) - mountain', '夜(よる/yoru) - night', '野菜(やさい/yasai) - vegetable'] },
    'yu': { romaji: 'yu', hiragana: 'ゆ', katakana: 'ユ', examples: ['雪(ゆき/yuki) - snow', '夢(ゆめ/yume) - dream', '指(ゆび/yubi) - finger'] },
    'yo': { romaji: 'yo', hiragana: 'よ', katakana: 'ヨ', examples: ['夜(よる/yoru) - night', '四(よん/yon) - four', '良い(よい/yoi) - good'] },
    'ra': { romaji: 'ra', hiragana: 'ら', katakana: 'ラ', examples: ['来週(らいしゅう/raishuu) - next week', '楽(らく/raku) - easy', 'ラーメン(raamen) - ramen'] },
    'ri': { romaji: 'ri', hiragana: 'り', katakana: 'リ', examples: ['料理(りょうり/ryouri) - cooking', '理由(りゆう/riyuu) - reason', 'リンゴ(ringo) - apple'] },
    'ru': { romaji: 'ru', hiragana: 'る', katakana: 'ル', examples: ['留学(りゅうがく/ryuugaku) - study abroad', 'ルール(ruuru) - rule', '昼(ひる/hiru) - noon'] },
    're': { romaji: 're', hiragana: 'れ', katakana: 'レ', examples: ['歴史(れきし/rekishi) - history', '冷蔵庫(れいぞうこ/reizouko) - refrigerator', 'レストラン(resutoran) - restaurant'] },
    'ro': { romaji: 'ro', hiragana: 'ろ', katakana: 'ロ', examples: ['六(ろく/roku) - six', 'ロボット(robotto) - robot', '廊下(ろうか/rouka) - corridor'] },
    'wa': { romaji: 'wa', hiragana: 'わ', katakana: 'ワ', examples: ['私(わたし/watashi) - I', '若い(わかい/wakai) - young', 'ワイン(wain) - wine'] },
    'wo': { romaji: 'wo', hiragana: 'を', katakana: 'ヲ', examples: ['を(wo) - object particle', '本を読む(ほんをよむ/hon wo yomu) - read a book', '水を飲む(みずをのむ/mizu wo nomu) - drink water'] },
    'n': { romaji: 'n', hiragana: 'ん', katakana: 'ン', examples: ['本(ほん/hon) - book', '今(いま/ima) - now', 'パン(pan) - bread'] }
};

// Chinese to Japanese Dictionary
const chineseDict = {
    '你好': 'こんにちは (konnichiwa)',
    '谢谢': 'ありがとう (arigatou)',
    '对不起': 'すみません (sumimasen)',
    '再见': 'さようなら (sayounara)',
    '早上好': 'おはよう (ohayou)',
    '晚上好': 'こんばんは (konbanwa)',
    '晚安': 'おやすみ (oyasumi)',
    '是': 'はい (hai)',
    '不是': 'いいえ (iie)',
    '请': 'お願いします (onegaishimasu)',
    '一': '一 (いち/ichi)',
    '二': '二 (に/ni)',
    '三': '三 (さん/san)',
    '四': '四 (よん/yon)',
    '五': '五 (ご/go)',
    '六': '六 (ろく/roku)',
    '七': '七 (なな/nana)',
    '八': '八 (はち/hachi)',
    '九': '九 (きゅう/kyuu)',
    '十': '十 (じゅう/juu)',
    '父亲': '父 (ちち/chichi)',
    '母亲': '母 (はは/haha)',
    '哥哥': '兄 (あに/ani)',
    '姐姐': '姉 (あね/ane)',
    '弟弟': '弟 (おとうと/otouto)',
    '妹妹': '妹 (いもうと/imouto)',
    '红色': '赤 (あか/aka)',
    '蓝色': '青 (あお/ao)',
    '白色': '白 (しろ/shiro)',
    '黑色': '黒 (くろ/kuro)',
    '黄色': '黄色 (きいろ/kiiro)',
    '绿色': '緑 (みどり/midori)',
    '春天': '春 (はる/haru)',
    '夏天': '夏 (なつ/natsu)',
    '秋天': '秋 (あき/aki)',
    '冬天': '冬 (ふゆ/fuyu)',
    '星期一': '月曜日 (げつようび/getsuyoubi)',
    '星期二': '火曜日 (かようび/kayoubi)',
    '星期三': '水曜日 (すいようび/suiyoubi)',
    '星期四': '木曜日 (もくようび/mokuyoubi)',
    '星期五': '金曜日 (きんようび/kinyoubi)',
    '星期六': '土曜日 (どようび/doyoubi)',
    '星期日': '日曜日 (にちようび/nichiyoubi)',
    '学习': '勉強 (べんきょう/benkyou)',
    '朋友': '友達 (ともだち/tomodachi)',
    '家': '家 (いえ/ie)',
    '学校': '学校 (がっこう/gakkou)',
    '老师': '先生 (せんせい/sensei)',
    '学生': '学生 (がくせい/gakusei)',
    '书': '本 (ほん/hon)',
    '水': '水 (みず/mizu)',
    '饭': 'ご飯 (ごはん/gohan)',
    '电话': '電話 (でんわ/denwa)',
    '时间': '時間 (じかん/jikan)',
    '今天': '今日 (きょう/kyou)',
    '明天': '明日 (あした/ashita)',
    '昨天': '昨日 (きのう/kinou)',
    '现在': '今 (いま/ima)',
    '早上': '朝 (あさ/asa)',
    '中午': '昼 (ひる/hiru)',
    '晚上': '夜 (よる/yoru)',
    '爱': '愛 (あい/ai)',
    '猫': '猫 (ねこ/neko)',
    '狗': '犬 (いぬ/inu)',
    '鱼': '魚 (さかな/sakana)',
    '鸟': '鳥 (とり/tori)',
    '花': '花 (はな/hana)',
    '树': '木 (き/ki)',
    '山': '山 (やま/yama)',
    '河': '川 (かわ/kawa)',
    '海': '海 (うみ/umi)'
};

// DOM Elements
const searchInput = document.getElementById('searchInput');
const searchBtn = document.getElementById('searchBtn');
const suggestionsDiv = document.getElementById('suggestions');
const resultContainer = document.getElementById('resultContainer');

// Event Listeners
searchInput.addEventListener('input', handleInput);
searchInput.addEventListener('keypress', (e) => {
    if (e.key === 'Enter') {
        performSearch();
    }
});
searchBtn.addEventListener('click', performSearch);

// Quick link buttons
document.querySelectorAll('.btn-link').forEach(btn => {
    btn.addEventListener('click', () => {
        const query = btn.getAttribute('data-query');
        searchInput.value = query;
        performSearch();
    });
});

// Handle input and show suggestions
function handleInput() {
    const query = searchInput.value.toLowerCase().trim();

    if (query.length === 0) {
        suggestionsDiv.classList.remove('show');
        return;
    }

    const suggestions = getSuggestions(query);

    if (suggestions.length > 0) {
        displaySuggestions(suggestions);
    } else {
        suggestionsDiv.classList.remove('show');
    }
}

// Get suggestions based on query
function getSuggestions(query) {
    const suggestions = [];

    // Search in romaji dictionary
    for (const [key, value] of Object.entries(romajiDict)) {
        if (key.startsWith(query)) {
            suggestions.push({
                type: 'romaji',
                key: key,
                value: value
            });
        }
    }

    // Search in Chinese dictionary
    for (const [key, value] of Object.entries(chineseDict)) {
        if (key.includes(query)) {
            suggestions.push({
                type: 'chinese',
                key: key,
                value: value
            });
        }
    }

    return suggestions.slice(0, 5); // Limit to 5 suggestions
}

// Display suggestions
function displaySuggestions(suggestions) {
    suggestionsDiv.innerHTML = suggestions.map(s => {
        if (s.type === 'romaji') {
            return `<div class="suggestion-item" onclick="selectSuggestion('${s.key}')">
                <strong>${s.key}</strong> → ${s.value.hiragana} (${s.value.katakana})
            </div>`;
        } else {
            return `<div class="suggestion-item" onclick="selectSuggestion('${s.key}')">
                <strong>${s.key}</strong> → ${s.value}
            </div>`;
        }
    }).join('');

    suggestionsDiv.classList.add('show');
}

// Select suggestion
function selectSuggestion(key) {
    searchInput.value = key;
    suggestionsDiv.classList.remove('show');
    performSearch();
}

// Perform search
function performSearch() {
    const query = searchInput.value.toLowerCase().trim();

    if (query.length === 0) {
        return;
    }

    suggestionsDiv.classList.remove('show');

    // Check romaji dictionary first
    if (romajiDict[query]) {
        displayFlashcard(romajiDict[query]);
        return;
    }

    // Check Chinese dictionary
    if (chineseDict[query]) {
        displayTranslation(query, chineseDict[query]);
        return;
    }

    // No match found
    displayError(query);
}

// Display flashcard for romaji
function displayFlashcard(data) {
    resultContainer.innerHTML = `
        <div class="flashcard">
            <div class="flashcard-header">
                <h2>🎴 日文假名卡片</h2>
                <p>Japanese Kana Flashcard</p>
            </div>
            <div class="flashcard-body">
                <div class="kana-section">
                    <div class="romaji-display">
                        Romaji: ${data.romaji.toUpperCase()}
                    </div>
                    <div class="kana-display">
                        <div class="kana-char">
                            <div class="kana-label">平假名 (Hiragana)</div>
                            <div class="kana-text">${data.hiragana}</div>
                        </div>
                        <div class="kana-char">
                            <div class="kana-label">片假名 (Katakana)</div>
                            <div class="kana-text">${data.katakana}</div>
                        </div>
                    </div>
                </div>
                <div class="examples">
                    <h3>📝 例词 (Example Words)</h3>
                    ${data.examples.map((ex, i) => `
                        <div class="example-item">${i + 1}. ${ex}</div>
                    `).join('')}
                </div>
            </div>
        </div>
    `;
}

// Display translation for Chinese
function displayTranslation(chinese, japanese) {
    resultContainer.innerHTML = `
        <div class="translation-result">
            <h2>📖 翻译结果</h2>
            <div class="chinese-text">${chinese}</div>
            <div style="font-size: 2rem; margin: 20px 0;">↓</div>
            <div class="japanese-text">${japanese.split('(')[0].trim()}</div>
            <div class="reading-text">${japanese.match(/\((.*?)\)/)[1]}</div>
        </div>
    `;
}

// Display error message
function displayError(query) {
    resultContainer.innerHTML = `
        <div class="error-message">
            <h3>😕 未找到匹配结果</h3>
            <p>找不到 "<strong>${query}</strong>" 的相关信息</p>
            <p>请尝试输入：</p>
            <ul style="list-style: none; margin-top: 20px;">
                <li>• 罗马字母，如：chi, tsu, ka, a</li>
                <li>• 中文词汇，如：你好, 谢谢, 再见</li>
            </ul>
        </div>
    `;
}

// Close suggestions when clicking outside
document.addEventListener('click', (e) => {
    if (!suggestionsDiv.contains(e.target) && e.target !== searchInput) {
        suggestionsDiv.classList.remove('show');
    }
});
