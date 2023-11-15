import { invoke } from "@tauri-apps/api/tauri";
import { useEffect, useState } from "react";
import { history } from './helpers/history'
import { Props } from "./types/props";
import { Navigate, useLocation } from "react-router-dom";

function FirstRunRouter({ children }: Props) {
    const location = useLocation()
    const [isFirstRun, setFirstRun] = useState(false)
    const [isLoading, setLoading] = useState(true)

    async function checkFirstRun() {
        let firstRun = await invoke("is_first_run") as boolean
        setFirstRun(firstRun);
        setLoading(false);
    }

    useEffect(() => {
        checkFirstRun()
    }, [])

    if (isLoading) {
        return <div>Loading...</div>; // Show a loading screen
    }

    if(isFirstRun) {
        if (location.pathname.includes('/firstrun')) return <>{ children }</>
        return <Navigate to='/firstrun' state={{ from: history.location }}></Navigate>
    }

    return <Navigate to='/home' state={{ from: history.location }}></Navigate>
}

export default FirstRunRouter;