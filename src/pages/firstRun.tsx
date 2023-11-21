import { Button, Card, CardContent, Grid, Typography } from "@mui/material";
import { t } from "i18next";
import ProviderSelect from "../components/providerSelect";
import { TbSubmarine } from "react-icons/tb";
import { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";


function FirstRun() {

    const navigate = useNavigate();
    const [adddedProviders, setAddedProviders] = useState([])

    useEffect(() => {
        setAddedProviders([]);
    }, []);


    function onAddSubsonicClick() {
        navigate("/addSubsonic")
    }

    function onDoneClick() {
        if (adddedProviders.length < 0) {
            navigate("/home")
        }
    }

    return (<><Card sx={{ marginBottom: 2 }}>
        <CardContent>
            <Typography color="text.secondary" sx={{ fontSize: 20, fontWeight: 10, marginBottom: 4 }}>
                {t('ManualAddProvider')}
            </Typography>
            <ProviderSelect onClick={onAddSubsonicClick}><TbSubmarine /><Typography sx={{ marginLeft: 2 }}>Subsonic / OpenSubsonic</Typography></ProviderSelect>
        </CardContent>
    </Card>
        <Card>
            <CardContent>
                <Typography color="text.secondary" sx={{ fontSize: 20, fontWeight: 10, marginBottom: 0 }}>
                    {t('AddedProviders')}
                </Typography>

                {adddedProviders.length === 0 ? <Typography sx={{ fontSize: 12 }}>{t('NoMediaProvidersAdded')}</Typography> : <></>}
            </CardContent>
        </Card>
        <Grid container justifyContent="flex-end" sx={{ position: "fixed", bottom: 20, right: 20 }}>
            <Button variant="contained" disabled onClick={onDoneClick}>
                <Typography>{t('Done')}</Typography>
            </Button>
        </Grid>
    </>)
}

export default FirstRun;