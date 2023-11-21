import { Box, Button, Card, Grid, TextField, Typography } from "@mui/material";
import { t } from "i18next";
import { useNavigate } from "react-router-dom";
import { history } from "../helpers/history";
import { TbSubmarine } from "react-icons/tb";
import React from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { toast } from 'react-toastify';

function AddSubsonic() {
    const navigate = useNavigate();

    function onBackClick() {
        navigate(history.location?.pathname ?? "/firstrun");
    }

    async function handleAddSubsonicSubmit(e: React.FormEvent<HTMLFormElement>) {
        e.preventDefault();
        
        const formData = new FormData(e.currentTarget);
        const host = formData.get("host")
        const username = formData.get("username")
        const password = formData.get("password")

        var correctCreds = await invoke("check_credentials", { provider: "Subsonic", host, username, password }) as unknown as boolean

        if(!correctCreds) {
            toast.error(t('InvalidCredentials'));
            return;
        }

        try {
        var addedProvider = await invoke("add_provider", { provider: "Subsonic", host, username, password }) as any
        }
        catch (exception){
            console.log(exception)
        }
        //navigate("/firstrun")
    }

    return (
        <Grid container direction="column" style={{ height: "100vh" }}>
            <Grid item sx={{ flexGrow: 1 }}>
                <Card sx={{ height: 'auto%', width: '75%', margin: 'auto' }}>
                    <Typography sx={{ textAlign: "center", fontSize: 50, marginTop: 5 }}><TbSubmarine /> Subsonic</Typography>

                    <Box component="form" onSubmit={handleAddSubsonicSubmit} noValidate
                        sx={{
                            marginTop: 8,
                            marginLeft: 10,
                            marginRight: 10,
                            display: 'flex',
                            flexDirection: 'column',
                            alignItems: 'center'
                        }}>
                        <TextField margin="normal" required fullWidth id="host" label="host" name="host" autoComplete="host" autoFocus />
                        <TextField margin="normal" required fullWidth id="username" label={t('Username')} name="username" autoComplete="username" />
                        <TextField margin="normal" required fullWidth id="password" label={t('Password')} name="password" autoComplete="password" type="password" />
                        <Button
                            type="submit"
                            fullWidth
                            variant="contained"
                            sx={{ mt: 3, mb: 5}}
                        >
                            {t('Add')}
                        </Button>
                    </Box>
                </Card>
            </Grid>
            <Grid item sx={{ position: "fixed", bottom: 20, left: 20 }}>
                <Button variant="contained" onClick={onBackClick}>
                    <Typography>{t('Back')}</Typography>
                </Button>
            </Grid>
        </Grid>
    );
}

export default AddSubsonic;
