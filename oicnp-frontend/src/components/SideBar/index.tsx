import { SITE as site } from '../../constants';

export const SideBar = () => {

  return (
    <aside className="g-sidebar-wrapper">
      <div className="g-sidebar">
        <section className="author-card">
          <a
            href="/2018/01/23/About-me.html"
            style={{ display: 'block' }}
          >
            <div className="avatar">
                <img src={`${site.url}${site.avatar}`} alt="" />
            </div>
            <div className="author-name">{site.author}</div>
          </a>
          <div className="bio">
              <p dangerouslySetInnerHTML={{ __html: site.bio }} />
          </div>

          {site.sns.length && (
            <ul id="sns-links" className="sns-links">
              {site.sns.map((snsItem) => {
                return (
                  <li key={snsItem.name}>
                    <a href={snsItem.url} target="_blank" rel="noreferrer">
                      <i className={`iconfont icon-${snsItem.name}`}></i>
                    </a>
                  </li>
                );
              })}
            </ul>
          )}
        </section>
      </div>

      <div className="search-card">
        <input id="search_input" type="text" placeholder="Search..." />
        <i className="iconfont icon-search" />
        <div className="search_result" />
      </div>
    </aside>
  );
}