import { highlight } from "@wikijump/prism"
import { defineElement } from "../util"

/**
 * FTML `[[code]]` element. Automatically highlights the contents of its
 * `<code>` child with Prism.
 */
export class Code extends HTMLDivElement {
  static tag = "wj-code"

  /** Observer for watching changes to the contents of the code element. */
  declare observer: MutationObserver

  /** The language highlighting is being done with. */
  declare language: string | null

  /** The current textual contents of this element. */
  declare content: string

  /** The compiled/highlighted HTML. */
  declare html?: string

  constructor() {
    super()

    this.language = null
    this.content = ""

    // observer for watching for changes to textual content
    this.observer = new MutationObserver(() => this.update())
  }

  /**
   * Extracts the language to highlight with from this elements classes.
   * Specifically, the `wj-language-{name}` class.
   */
  private getLanguageFromClass() {
    const classes = Array.from(this.classList)
    for (const name of classes) {
      // this will always be ASCII lowercased,
      // so we can just use a simple check
      if (name.startsWith("wj-language-")) return name.substr(12)
    }
    return null
  }

  /** Ran whenever highlighting needs to be updated. */
  private update() {
    // get the element every time we update,
    // because it might have been replaced by morphing or something
    const element = this.querySelector("code")
    if (!element) return

    this.observer.disconnect()

    const language = this.getLanguageFromClass() ?? "none"
    const content = element.innerText

    // don't waste resources if we're just doing the same thing
    if (!this.html || this.content !== content || this.language !== language) {
      this.language = language
      this.content = content
      this.html = highlight(content, language)
    }

    element.innerHTML = this.html

    this.observer.observe(this, {
      characterData: true,
      childList: true,
      subtree: true
    })
  }

  // -- LIFECYCLE

  connectedCallback() {
    if (!this.querySelector("pre")) {
      const defaultElement = document.createElement("pre")
      defaultElement.append(document.createElement("code"))
      this.appendChild(defaultElement)
    }

    this.update()
  }

  adoptedCallback() {
    this.update()
  }

  attributeChangedCallback() {
    this.update()
  }
}

defineElement(Code.tag, Code, { extends: "div" })
