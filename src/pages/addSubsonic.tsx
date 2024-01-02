import { Button, Card, Flex, FormControl, FormErrorMessage, FormLabel, Input, Text, VStack } from "@chakra-ui/react";
import { t } from "i18next";
import { useNavigate } from "react-router-dom";
import { history } from "../helpers/history";
import { TbSubmarine } from "react-icons/tb";
import { invoke } from "@tauri-apps/api/tauri";
import { toast } from 'react-toastify';
import { Field, Formik } from "formik";

function AddSubsonic() {
    const navigate = useNavigate();

    function onBackClick() {
        navigate(history.location?.pathname ?? "/firstrun");
    }

    async function handleAddSubsonicSubmit(values: {
        host: string;
        username: string;
        password: string;
    }) {
        const host = values.host;
        const username = values.username;
        const password = values.password;

        var correctCreds = await invoke("check_credentials", { provider: "Subsonic", host, username, password }) as unknown as boolean

        if (!correctCreds) {
            toast.error(t('InvalidCredentials'));
            return;
        }

        try {
            var addedProvider = await invoke("add_provider", { provider: "Subsonic", host, username, password }) as any
        }
        catch (exception) {
            console.log(exception)
        }

        if (addedProvider) {
            navigate("/firstrun")
        }
    }

    return (
        <>
            <Flex align="center" justify="center" h="100vh">
                <Card sx={{ height: 'auto%', width: '75%', margin: 'auto' }}>
                    <Text sx={{ textAlign: "center", fontSize: 50, marginTop: 5 }}><TbSubmarine /> Subsonic</Text>
                    <Formik
                        initialValues={{
                            host: "",
                            username: "",
                            password: ""
                        }}
                        onSubmit={handleAddSubsonicSubmit}
                    >
                        {({ handleSubmit, errors, touched }) => (
                            <form onSubmit={handleSubmit}>
                                <VStack spacing={4} align="flex-start">
                                    <FormControl isInvalid={!!errors.host && touched.host}>
                                        <FormLabel htmlFor="host">{t('Host')}</FormLabel>
                                        <Field
                                            as={Input}
                                            color="black"
                                            id="host"
                                            name="host"
                                            type="text"
                                            variant="filled"
                                            validate={(value: string) => {
                                                let error;

                                                if (value.length === 0) {
                                                    error = t("FieldRequired") //TODO translate
                                                }

                                                return error;
                                            }} />
                                        <FormErrorMessage>{errors.host}</FormErrorMessage>
                                    </FormControl>
                                    <FormControl>
                                        <FormLabel htmlFor="username">{t('Username')}</FormLabel>
                                        <Field
                                            as={Input}
                                            color="black"
                                            id="username"
                                            name="username"
                                            type="text"
                                            variant="filled"
                                            validate={(value: string) => {
                                                let error;

                                                if (value.length === 0) {
                                                    error = t("FieldRequired") //TODO translate
                                                }

                                                return error;
                                            }} />
                                        <FormErrorMessage>{errors.username}</FormErrorMessage>
                                    </FormControl>
                                    <FormControl>
                                        <FormLabel htmlFor="password">{t('Password')}</FormLabel>
                                        <Field
                                            as={Input}
                                            color="black"
                                            id="password"
                                            name="password"
                                            type="password"
                                            variant="filled"
                                            validate={(value: string) => {
                                                let error;

                                                if (value.length === 0) {
                                                    error = t("FieldRequired") //TODO translate
                                                }

                                                return error;
                                            }} />
                                        <FormErrorMessage>{errors.password}</FormErrorMessage>
                                    </FormControl>
                                    <Button type="submit" colorScheme="blue" width="full">
                                        {t('Add')}
                                    </Button>
                                </VStack>
                            </form>
                        )}
                    </Formik>

                </Card>
            </Flex>
            <Flex position="fixed" bottom="10" left="10">
                <Button colorScheme="blue" onClick={onBackClick}>
                    {t('Back')}
                </Button>
            </Flex>
        </>
    );
}

export default AddSubsonic;
