import { invoke } from "@tauri-apps/api/tauri"
import { useEffect } from "react"

function Home() {

    useEffect(() => {
        invoke("is_first_scan").then(result => {
            if (result as unknown as boolean === true) {
                invoke("scan_providers")
            }
        })
    }, [])

    return (<></>)
}

export default Home