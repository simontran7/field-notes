(function () {
  function currentTheme() {
    return document.documentElement.getAttribute('data-theme') || 'light'
  }

  window.__toggleTheme = function () {
    var next = currentTheme() === 'dark' ? 'light' : 'dark'
    var system = window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light'
    if (next === system) {
      localStorage.removeItem('theme-manual')
    } else {
      localStorage.setItem('theme-manual', next)
    }
    document.documentElement.setAttribute('data-theme', next)
  }

  window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', function (e) {
    if (!localStorage.getItem('theme-manual')) {
      document.documentElement.setAttribute('data-theme', e.matches ? 'dark' : 'light')
    }
  })
})()

const COPY_ICON = '<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="8" y="2" width="8" height="4" rx="1" ry="1"></rect><path d="M16 4h2a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h2"></path></svg>'
const CHECK_ICON = '<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="20 6 9 17 4 12"></polyline></svg>'

document.addEventListener('DOMContentLoaded', function () {
  document.querySelectorAll('.email-link').forEach(function (a) {
    a.href = 'mailto:' + a.dataset.user + '@' + a.dataset.domain
  })

  document.querySelectorAll('.prose pre code.language-mermaid').forEach(function (code) {
    const div = document.createElement('div')
    div.className = 'mermaid'
    div.textContent = code.textContent
    code.parentElement.replaceWith(div)
  })

  if (window.mermaid) {
    const theme = document.documentElement.getAttribute('data-theme') === 'dark' ? 'dark' : 'default'
    mermaid.initialize({ startOnLoad: false, theme: theme })
    mermaid.run({ querySelector: '.mermaid' })
  }

  // Copy button on every highlighted code block (scripts/highlight.mjs wraps them in .code-block)
  document.querySelectorAll('.code-block').forEach(function (block) {
    // Both color variants hold the same text, so either one can be copied
    var pre = block.querySelector('pre')
    var btn = document.createElement('button')
    btn.className = 'copy-btn'
    btn.setAttribute('aria-label', 'Copy code')
    btn.innerHTML = COPY_ICON
    btn.addEventListener('click', function () {
      navigator.clipboard.writeText(pre.textContent).then(function () {
        btn.innerHTML = CHECK_ICON
        setTimeout(function () { btn.innerHTML = COPY_ICON }, 2000)
      })
    })
    block.appendChild(btn)
  })

  if (window.katex) {
    document.querySelectorAll('[data-math-style]').forEach(function (el) {
      window.katex.render(el.textContent, el, {
        displayMode: el.dataset.mathStyle === 'display',
        throwOnError: false,
      })
    })
  }
})
