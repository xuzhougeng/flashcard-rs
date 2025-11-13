// Tauri API - check if available
let invoke;
if (window.__TAURI__ && window.__TAURI__.core) {
    invoke = window.__TAURI__.core.invoke;
} else {
    // Fallback for testing without Tauri
    invoke = async (cmd, args) => {
        console.log('Mock invoke:', cmd, args);
        if (cmd === 'get_settings') {
            return { interval: 10, autostart: false, card_type: 'mixed' };
        }
        return null;
    };
}

// Romaji to Japanese Dictionary (same as web version)
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
    '你好': { chinese: '你好', japanese: 'こんにちは', reading: 'konnichiwa' },
    '谢谢': { chinese: '谢谢', japanese: 'ありがとう', reading: 'arigatou' },
    '对不起': { chinese: '对不起', japanese: 'すみません', reading: 'sumimasen' },
    '再见': { chinese: '再见', japanese: 'さようなら', reading: 'sayounara' },
    '早上好': { chinese: '早上好', japanese: 'おはよう', reading: 'ohayou' },
    '晚上好': { chinese: '晚上好', japanese: 'こんばんは', reading: 'konbanwa' },
    '晚安': { chinese: '晚安', japanese: 'おやすみ', reading: 'oyasumi' },
    '是': { chinese: '是', japanese: 'はい', reading: 'hai' },
    '不是': { chinese: '不是', japanese: 'いいえ', reading: 'iie' },
    '请': { chinese: '请', japanese: 'お願いします', reading: 'onegaishimasu' },
    '一': { chinese: '一', japanese: '一', reading: 'いち/ichi' },
    '二': { chinese: '二', japanese: '二', reading: 'に/ni' },
    '三': { chinese: '三', japanese: '三', reading: 'さん/san' },
    '四': { chinese: '四', japanese: '四', reading: 'よん/yon' },
    '五': { chinese: '五', japanese: '五', reading: 'ご/go' },
    '六': { chinese: '六', japanese: '六', reading: 'ろく/roku' },
    '七': { chinese: '七', japanese: '七', reading: 'なな/nana' },
    '八': { chinese: '八', japanese: '八', reading: 'はち/hachi' },
    '九': { chinese: '九', japanese: '九', reading: 'きゅう/kyuu' },
    '十': { chinese: '十', japanese: '十', reading: 'じゅう/juu' },
    '父亲': { chinese: '父亲', japanese: '父', reading: 'ちち/chichi' },
    '母亲': { chinese: '母亲', japanese: '母', reading: 'はは/haha' },
    '哥哥': { chinese: '哥哥', japanese: '兄', reading: 'あに/ani' },
    '姐姐': { chinese: '姐姐', japanese: '姉', reading: 'あね/ane' },
    '弟弟': { chinese: '弟弟', japanese: '弟', reading: 'おとうと/otouto' },
    '妹妹': { chinese: '妹妹', japanese: '妹', reading: 'いもうと/imouto' },
    '红色': { chinese: '红色', japanese: '赤', reading: 'あか/aka' },
    '蓝色': { chinese: '蓝色', japanese: '青', reading: 'あお/ao' },
    '白色': { chinese: '白色', japanese: '白', reading: 'しろ/shiro' },
    '黑色': { chinese: '黑色', japanese: '黒', reading: 'くろ/kuro' },
    '黄色': { chinese: '黄色', japanese: '黄色', reading: 'きいろ/kiiro' },
    '绿色': { chinese: '绿色', japanese: '緑', reading: 'みどり/midori' },
    '春天': { chinese: '春天', japanese: '春', reading: 'はる/haru' },
    '夏天': { chinese: '夏天', japanese: '夏', reading: 'なつ/natsu' },
    '秋天': { chinese: '秋天', japanese: '秋', reading: 'あき/aki' },
    '冬天': { chinese: '冬天', japanese: '冬', reading: 'ふゆ/fuyu' },
    '星期一': { chinese: '星期一', japanese: '月曜日', reading: 'げつようび/getsuyoubi' },
    '星期二': { chinese: '星期二', japanese: '火曜日', reading: 'かようび/kayoubi' },
    '星期三': { chinese: '星期三', japanese: '水曜日', reading: 'すいようび/suiyoubi' },
    '星期四': { chinese: '星期四', japanese: '木曜日', reading: 'もくようび/mokuyoubi' },
    '星期五': { chinese: '星期五', japanese: '金曜日', reading: 'きんようび/kinyoubi' },
    '星期六': { chinese: '星期六', japanese: '土曜日', reading: 'どようび/doyoubi' },
    '星期日': { chinese: '星期日', japanese: '日曜日', reading: 'にちようび/nichiyoubi' },
    '学习': { chinese: '学习', japanese: '勉強', reading: 'べんきょう/benkyou' },
    '朋友': { chinese: '朋友', japanese: '友達', reading: 'ともだち/tomodachi' },
    '家': { chinese: '家', japanese: '家', reading: 'いえ/ie' },
    '学校': { chinese: '学校', japanese: '学校', reading: 'がっこう/gakkou' },
    '老师': { chinese: '老师', japanese: '先生', reading: 'せんせい/sensei' },
    '学生': { chinese: '学生', japanese: '学生', reading: 'がくせい/gakusei' },
    '书': { chinese: '书', japanese: '本', reading: 'ほん/hon' },
    '水': { chinese: '水', japanese: '水', reading: 'みず/mizu' },
    '饭': { chinese: '饭', japanese: 'ご飯', reading: 'ごはん/gohan' },
    '电话': { chinese: '电话', japanese: '電話', reading: 'でんわ/denwa' },
    '时间': { chinese: '时间', japanese: '時間', reading: 'じかん/jikan' },
    '今天': { chinese: '今天', japanese: '今日', reading: 'きょう/kyou' },
    '明天': { chinese: '明天', japanese: '明日', reading: 'あした/ashita' },
    '昨天': { chinese: '昨天', japanese: '昨日', reading: 'きのう/kinou' },
    '现在': { chinese: '现在', japanese: '今', reading: 'いま/ima' },
    '早上': { chinese: '早上', japanese: '朝', reading: 'あさ/asa' },
    '中午': { chinese: '中午', japanese: '昼', reading: 'ひる/hiru' },
    '晚上': { chinese: '晚上', japanese: '夜', reading: 'よる/yoru' },
    '爱': { chinese: '爱', japanese: '愛', reading: 'あい/ai' },
    '猫': { chinese: '猫', japanese: '猫', reading: 'ねこ/neko' },
    '狗': { chinese: '狗', japanese: '犬', reading: 'いぬ/inu' },
    '鱼': { chinese: '鱼', japanese: '魚', reading: 'さかな/sakana' },
    '鸟': { chinese: '鸟', japanese: '鳥', reading: 'とり/tori' },
    '花': { chinese: '花', japanese: '花', reading: 'はな/hana' },
    '树': { chinese: '树', japanese: '木', reading: 'き/ki' },
    '山': { chinese: '山', japanese: '山', reading: 'やま/yama' },
    '河': { chinese: '河', japanese: '川', reading: 'かわ/kawa' },
    '海': { chinese: '海', japanese: '海', reading: 'うみ/umi' }
};

// DOM Elements
const cardContainer = document.getElementById('cardContainer');
const refreshBtn = document.getElementById('refreshBtn');
const settingsBtn = document.getElementById('settingsBtn');
const settingsModal = document.getElementById('settingsModal');
const closeModalBtn = document.querySelector('.close');
const saveSettingsBtn = document.getElementById('saveSettingsBtn');
const intervalSelect = document.getElementById('intervalSelect');
const autostartCheckbox = document.getElementById('autostartCheckbox');
const cardTypeSelect = document.getElementById('cardTypeSelect');

// Load settings on startup
async function loadSettings() {
    try {
        const settings = await invoke('get_settings');
        intervalSelect.value = settings.interval;
        autostartCheckbox.checked = settings.autostart;
        cardTypeSelect.value = settings.card_type;
    } catch (error) {
        console.error('Failed to load settings:', error);
    }
}

// Save settings
async function saveSettings() {
    try {
        const settings = {
            interval: parseInt(intervalSelect.value),
            autostart: autostartCheckbox.checked,
            card_type: cardTypeSelect.value
        };
        await invoke('save_settings', { settings });
        alert('设置已保存！');
        settingsModal.classList.remove('show');
    } catch (error) {
        console.error('Failed to save settings:', error);
        alert('保存设置失败！');
    }
}

// Get random card with safe fallback
function getRandomCard() {
    // Get card type, fallback to mixed if settings not loaded yet
    const cardType = cardTypeSelect ? cardTypeSelect.value : 'mixed';

    if (cardType === 'romaji') {
        return getRandomRomajiCard();
    } else if (cardType === 'chinese') {
        return getRandomChineseCard();
    } else {
        // Mixed: 50% romaji, 50% chinese
        return Math.random() < 0.5 ? getRandomRomajiCard() : getRandomChineseCard();
    }
}

function getRandomRomajiCard() {
    const keys = Object.keys(romajiDict);
    const randomKey = keys[Math.floor(Math.random() * keys.length)];
    return { type: 'romaji', data: romajiDict[randomKey] };
}

function getRandomChineseCard() {
    const keys = Object.keys(chineseDict);
    const randomKey = keys[Math.floor(Math.random() * keys.length)];
    return { type: 'chinese', data: chineseDict[randomKey] };
}

// Display flashcard
function displayCard(card) {
    if (card.type === 'romaji') {
        displayRomajiCard(card.data);
    } else {
        displayChineseCard(card.data);
    }
}

function displayRomajiCard(data) {
    cardContainer.innerHTML = `
        <div class="card-flip-container" onclick="flipCard()">
            <div class="flashcard">
                <!-- Front side: Question (Romaji) -->
                <div class="card-front">
                    <div class="flashcard-header">
                        <h2>🎴 日文假名卡片</h2>
                        <p>Japanese Kana Flashcard</p>
                    </div>
                    <div class="question-display">
                        <div class="question-label">罗马字 Romaji</div>
                        <div class="question-text">${data.romaji.toUpperCase()}</div>
                    </div>
                    <div class="flip-hint">💡 点击或按空格键查看答案</div>
                </div>

                <!-- Back side: Answer (Hiragana, Katakana, Examples) -->
                <div class="card-back">
                    <div class="flashcard-header">
                        <h2>🎴 答案</h2>
                        <p>Answer</p>
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
            </div>
        </div>
    `;
}

function displayChineseCard(data) {
    cardContainer.innerHTML = `
        <div class="card-flip-container" onclick="flipCard()">
            <div class="translation-card">
                <!-- Front side: Question (Chinese) -->
                <div class="card-front">
                    <div class="flashcard-header">
                        <h2>📖 中日翻译卡片</h2>
                        <p>Chinese-Japanese Translation</p>
                    </div>
                    <div class="question-display">
                        <div class="question-label">中文词汇 Chinese</div>
                        <div class="question-text">${data.chinese}</div>
                    </div>
                    <div class="flip-hint">💡 点击或按空格键查看答案</div>
                </div>

                <!-- Back side: Answer (Japanese, Reading) -->
                <div class="card-back">
                    <div class="flashcard-header">
                        <h2>📖 答案</h2>
                        <p>Answer</p>
                    </div>
                    <div class="translation-body">
                        <div class="chinese-text">${data.chinese}</div>
                        <div class="arrow">↓</div>
                        <div class="japanese-text">${data.japanese}</div>
                        <div class="reading-text">${data.reading}</div>
                    </div>
                </div>
            </div>
        </div>
    `;
}

// Flip card function
function flipCard() {
    const flashcard = document.querySelector('.flashcard, .translation-card');
    if (flashcard) {
        flashcard.classList.toggle('flipped');
    }
}

// Event Listeners
refreshBtn.addEventListener('click', () => {
    const card = getRandomCard();
    displayCard(card);
});

// Keyboard event for spacebar flip
document.addEventListener('keydown', (e) => {
    // Only flip card if:
    // 1. Spacebar is pressed
    // 2. Settings modal is not showing
    // 3. User is not typing in an input/textarea/select element
    const isInputElement = e.target.tagName === 'INPUT' ||
                          e.target.tagName === 'TEXTAREA' ||
                          e.target.tagName === 'SELECT';

    if (e.code === 'Space' && !settingsModal.classList.contains('show') && !isInputElement) {
        e.preventDefault(); // Prevent page scroll
        flipCard();
    }
});

settingsBtn.addEventListener('click', () => {
    settingsModal.classList.add('show');
});

closeModalBtn.addEventListener('click', () => {
    settingsModal.classList.remove('show');
});

saveSettingsBtn.addEventListener('click', saveSettings);

// Close modal when clicking outside
settingsModal.addEventListener('click', (e) => {
    if (e.target === settingsModal) {
        settingsModal.classList.remove('show');
    }
});

// Initialize
async function init() {
    try {
        // Wait for settings to load before displaying first card
        await loadSettings();
    } catch (error) {
        console.error('Failed to load settings, using defaults:', error);
    }

    // Now that settings are loaded, display the card
    const card = getRandomCard();
    displayCard(card);
}

// Call init when DOM is ready
if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', init);
} else {
    init();
}
