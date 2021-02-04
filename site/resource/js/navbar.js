const l = window.location.pathname;
console.log(l);

const nav = `<div id="navbar" style="border-color: #3634d2; border-width: 2px; border-style: solid; border-radius: 4px;">
<header class="navbar" style="margin: 1rem;">
    <section class="navbar-section">
        <a href="/resume" class="btn btn-link" ${(() => {
            return l == '/resume' ? `disabled` : 'enabled'
        })()}>Resume</a>
        <a href="resource/md/blog.md" class="btn btn-link" ${(() => {
            return l == '/blog' ? `disabled` : 'enabled'
        })()}>Blog</a>
        <a href="resource/html/contact.html" class="btn btn-link" ${(() => {
            return l == '/contact' ? `disabled` : 'enabled'
        })()}>Contact</a>
    </section>
    <section class="navbar-center">
        <a href="${(() => { return l == '/index.html' ? '#' : '/index.html' })()}" class="navbar-brand mr-2 text-bold">James J Meyer</a>
    </section>
    <section class="navbar-section">
        <div class="input-group input-inline">
            <input class="form-input" type="text" placeholder="search">
            <button class="btn btn-primary input-group-btn">Search</button>
        </div>
    </section>
</header>
</div>`;

document.getElementById('navbar').innerHTML = nav;