import { Button, Card, CardBody, Flex, Text } from "@chakra-ui/react";
import { t } from "i18next";
import ProviderSelect from "../components/providerSelect";
import { TbSubmarine } from "react-icons/tb";
import { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import { invoke } from "@tauri-apps/api/tauri";
import Provider from "../models/provider";

function FirstRun() {

    const navigate = useNavigate();
    const [adddedProviders, setAddedProviders] = useState<Provider[]>([])

    useEffect(() => {
        invoke("get_providers").then((providers) => {
            setAddedProviders(providers as unknown as Provider[]);
        })
    }, []);


    function onAddSubsonicClick() {
        navigate("/addSubsonic")
    }

    function onDoneClick() {
        if (adddedProviders.length > 0) {
            navigate("/home")
        }
    }

    return (<><Card sx={{ marginBottom: 2 }}>
        <CardBody>
            <Text color="text.secondary" sx={{ fontSize: 20, fontWeight: 10, marginBottom: 4 }}>
                {t('ManualAddProvider')}
            </Text>
            <ProviderSelect onClick={onAddSubsonicClick}><TbSubmarine /><Text sx={{ marginLeft: 2 }}>Subsonic / OpenSubsonic</Text></ProviderSelect>
        </CardBody>
    </Card>
        <Card>
            <CardBody>
                <Text color="text.secondary" sx={{ fontSize: 20, fontWeight: 10, marginBottom: 0 }}>
                    {t('AddedProviders')}
                </Text>

                {adddedProviders.length === 0 ? <Text sx={{ fontSize: 12 }}>{t('NoMediaProvidersAdded')}</Text> : <>{adddedProviders.map((providers) => (
                    <div>
                        <p>{providers.name}</p>
                        <p>{providers.ip}</p>
                    </div>
                ))}</>}
            </CardBody>
        </Card>
        <Flex justifyContent="flex-end" sx={{ position: "fixed", bottom: 20, right: 20 }}>
            <Button colorScheme={adddedProviders.length === 0 ? "gray" : "blue"} disabled={adddedProviders.length === 0} onClick={onDoneClick}
                sx={{
                    pointerEvents: adddedProviders.length === 0 ? "none" : "auto",
                    cursor: adddedProviders.length === 0 ? "not-allowed" : "pointer"
                }}>
                <Text>{t('Done')}</Text>
            </Button>
        </Flex>
    </>)
}

export default FirstRun;