(() => {
    var e = async (t) => {
      await (
        await t()
      )();
    };
    (self.Astro || (self.Astro = {})).only = e;
    window.dispatchEvent(new Event("astro:only"));
  })();
  (() => {
    var d;
    {
      let p = {
          0: (t) => u(t),
          1: (t) => l(t),
          2: (t) => new RegExp(t),
          3: (t) => new Date(t),
          4: (t) => new Map(l(t)),
          5: (t) => new Set(l(t)),
          6: (t) => BigInt(t),
          7: (t) => new URL(t),
          8: (t) => new Uint8Array(t),
          9: (t) => new Uint16Array(t),
          10: (t) => new Uint32Array(t),
        },
        h = (t) => {
          let [e, n] = t;
          return e in p ? p[e](n) : void 0;
        },
        l = (t) => t.map(h),
        u = (t) =>
          typeof t != "object" || t === null
            ? t
            : Object.fromEntries(
                Object.entries(t).map(([e, n]) => [e, h(n)])
              );
      customElements.get("astro-island") ||
        customElements.define(
          "astro-island",
          ((d = class extends HTMLElement {
            constructor() {
              super(...arguments);
              this.hydrate = async () => {
                var i;
                if (!this.hydrator || !this.isConnected) return;
                let e =
                  (i = this.parentElement) == null
                    ? void 0
                    : i.closest("astro-island[ssr]");
                if (e) {
                  e.addEventListener("astro:hydrate", this.hydrate, {
                    once: !0,
                  });
                  return;
                }
                let n = this.querySelectorAll("astro-slot"),
                  o = {},
                  a = this.querySelectorAll(
                    "template[data-astro-template]"
                  );
                for (let r of a) {
                  let s = r.closest(this.tagName);
                  s != null &&
                    s.isSameNode(this) &&
                    ((o[
                      r.getAttribute("data-astro-template") || "default"
                    ] = r.innerHTML),
                    r.remove());
                }
                for (let r of n) {
                  let s = r.closest(this.tagName);
                  s != null &&
                    s.isSameNode(this) &&
                    (o[r.getAttribute("name") || "default"] =
                      r.innerHTML);
                }
                let c;
                try {
                  c = this.hasAttribute("props")
                    ? u(JSON.parse(this.getAttribute("props")))
                    : {};
                } catch (r) {
                  let s =
                      this.getAttribute("component-url") || "<unknown>",
                    y = this.getAttribute("component-export");
                  throw (
                    (y && (s += ` (export ${y})`),
                    console.error(
                      `[hydrate] Error parsing props for component ${s}`,
                      this.getAttribute("props"),
                      r
                    ),
                    r)
                  );
                }
                await this.hydrator(this)(this.Component, c, o, {
                  client: this.getAttribute("client"),
                }),
                  this.removeAttribute("ssr"),
                  this.dispatchEvent(new CustomEvent("astro:hydrate"));
              };
            }
            connectedCallback() {
              !this.hasAttribute("await-children") || this.firstChild
                ? this.childrenConnectedCallback()
                : new MutationObserver((e, n) => {
                    n.disconnect(),
                      setTimeout(
                        () => this.childrenConnectedCallback(),
                        0
                      );
                  }).observe(this, { childList: !0 });
            }
            async childrenConnectedCallback() {
              let e = this.getAttribute("before-hydration-url");
              e && (await import(e)), this.start();
            }
            start() {
              let e = JSON.parse(this.getAttribute("opts")),
                n = this.getAttribute("client");
              if (Astro[n] === void 0) {
                window.addEventListener(
                  `astro:${n}`,
                  () => this.start(),
                  { once: !0 }
                );
                return;
              }
              Astro[n](
                async () => {
                  let o = this.getAttribute("renderer-url"),
                    [a, { default: c }] = await Promise.all([
                      import(this.getAttribute("component-url")),
                      o ? import(o) : () => () => {},
                    ]),
                    i =
                      this.getAttribute("component-export") || "default";
                  if (!i.includes(".")) this.Component = a[i];
                  else {
                    this.Component = a;
                    for (let r of i.split("."))
                      this.Component = this.Component[r];
                  }
                  return (this.hydrator = c), this.hydrate;
                },
                e,
                this
              );
            }
            attributeChangedCallback() {
              this.hydrate();
            }
          }),
          (d.observedAttributes = ["props"]),
          d)
        );
    }
  })();