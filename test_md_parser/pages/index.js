import styles from '../styles/Home.module.css'
import { useState } from 'react'
import { render_html } from "../../pkg/tiny_md_parser_bg"

const INITIAL_MD = '# goes here'

export default function Home() {

  const [mdtext, setMdtext] = useState(INITIAL_MD)
  const [htmltext, setHtmltext] = useState(render_html(INITIAL_MD))
  return (
    <div className={styles.container}>
      <div className={styles.half}>
        markdown text:
        <button onClick={
          () => {
            setHtmltext(render_html(mdtext))
          }
        }>parse</button>
        <textarea
        className={styles.md_area}
          value={mdtext}
          onChange={(e) => setMdtext(e.target.value)}
        />
      </div>
      <div dangerouslySetInnerHTML={{ __html: htmltext }} />
    </div>
  )
}
