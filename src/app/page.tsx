const currentWords = ["비상구", "감", "첫눈"];

const sampleStories = [
  {
    title: "첫눈이 내리던 층",
    author: "minho",
    body: "비상구 문틈으로 첫눈이 들어왔다. 손에 쥔 감은 아직 차갑지 않았고, 나는 그 겨울이 아직 시작되지 않았다고 믿기로 했다.",
    meta: "오늘의 세 단어",
  },
  {
    title: "익은 감의 냄새",
    author: "sambook",
    body: "모두가 엘리베이터를 기다릴 때 아이는 비상구로 뛰었다. 계단참에는 감 하나가 놓여 있었고, 창밖에는 첫눈이 조용히 쌓이고 있었다.",
    meta: "같은 단어 조합",
  },
];

const checks = currentWords.map((word) => ({
  word,
  hit: sampleStories[0].body.includes(word),
}));

export default function Home() {
  return (
    <main className="page">
      <div className="workspace">
        <header className="topbar">
          <div className="brand">
            <div className="brand-name">Sambook</div>
            <div className="brand-subtitle">세 단어로 시작하는 짧은 이야기</div>
          </div>
          <nav className="top-actions" aria-label="주요 메뉴">
            <button className="button ghost" type="button">
              공개 글
            </button>
            <button className="button primary" type="button">
              쓰기
            </button>
          </nav>
        </header>

        <section className="hero" aria-labelledby="hero-title">
          <div className="intro">
            <div className="eyebrow">오늘의 세 단어</div>
            <h1 className="headline" id="hero-title">
              같은 단어에서 다른 이야기가 태어납니다.
            </h1>
            <p className="summary">
              무작위로 받은 한글 단어 세 개를 붙잡고 짧은 이야기를 씁니다.
              공개하면 같은 단어로 출발한 다른 사람의 글도 함께 볼 수 있습니다.
            </p>
            <div className="word-board" aria-label="현재 단어 세 개">
              {currentWords.map((word) => (
                <div className="word-tile" key={word}>
                  <span className="word-text">{word}</span>
                </div>
              ))}
            </div>
            <div className="top-actions">
              <button className="button primary" type="button">
                이 단어로 쓰기
              </button>
              <button className="button" type="button">
                다시 뽑기
              </button>
            </div>
          </div>

          <aside className="editor-shell" aria-label="이야기 작성 미리보기">
            <div className="editor-header">
              <h2 className="editor-title">짧은 이야기</h2>
              <div className="visibility" aria-label="공개 범위">
                <span className="active">공개</span>
                <span>비공개</span>
              </div>
            </div>
            <input
              className="title-input"
              defaultValue="첫눈이 내리던 층"
              aria-label="제목"
            />
            <textarea
              className="story-input"
              defaultValue={sampleStories[0].body}
              aria-label="본문"
            />
            <div className="editor-footer">
              <div className="word-checks" aria-label="단어 포함 여부">
                {checks.map((check) => (
                  <span
                    className={check.hit ? "word-check hit" : "word-check"}
                    key={check.word}
                  >
                    {check.word}
                  </span>
                ))}
              </div>
              <span>{sampleStories[0].body.length} / 300자</span>
            </div>
          </aside>
        </section>

        <section className="sections">
          <div>
            <h2 className="section-title">공개 글</h2>
            <div className="feed">
              {sampleStories.map((story) => (
                <article className="story-card" key={story.title}>
                  <h3>{story.title}</h3>
                  <p>{story.body}</p>
                  <div className="story-meta">
                    <span>{story.author}</span>
                    <span>{story.meta}</span>
                  </div>
                </article>
              ))}
            </div>
          </div>

          <div>
            <h2 className="section-title">초기 구조</h2>
            <ul className="system-list">
              <li>단어장은 큰 한글 seed에서 시작하고 품사, 분위기, 장르 태그를 붙입니다.</li>
              <li>글은 공개 또는 비공개로 저장하며, 공개 글은 같은 세 단어별로 묶습니다.</li>
              <li>매일 하나의 공통 프롬프트를 제공해 흩어진 랜덤 글쓰기를 하나의 광장으로 모읍니다.</li>
            </ul>
          </div>
        </section>
      </div>
    </main>
  );
}

