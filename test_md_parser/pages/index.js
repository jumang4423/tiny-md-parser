import styles from '../styles/Home.module.css'
import { useState } from 'react'
import { render_md } from "../../pkg/tiny_md_parser_bg"

export default function Home() {

  const [mdtext, setMdtext] = useState('# markdown')
  const [htmltext, setHtmltext] = useState('')
  console.log(render_md("# ass"));
  return (
    <div className={styles.container}>
      <div className={styles.half}>
        markdown text:
        <button onClick={
          () => {
            setHtmltext(render_md(mdtext))
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
