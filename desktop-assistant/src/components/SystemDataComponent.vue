<template>
    <div id="inner-data">
        <p>System Data</p>
        <p>{{ greetMsg }}</p>
    </div>

</template>

<script>
    import { invoke } from "@tauri-apps/api/tauri";

    export default{
        data(){
            return{
                greetMsg: "",
                name: "Kay"
            }
        },
        mounted(){

            console.log("System Data:")
            this.name = "lolol"
            this.greet()
            
            this.test_invoke("Gary")
        },
        methods:{
            async greet(){
                this.greetMsg = await invoke("greet", { name: this.name });
            },
            async test_invoke(input_name){
                console.log(await invoke("test_invoke", { name: input_name }));
            }
            /*
            async function greet() {
                // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
                greetMsg.value = await invoke("greet", { name: name.value });
            }
            */
        }
    }
</script>

<style>
    #inner-data{
        background-color: white;

        margin-right: 15px;

        padding-left: 5px;
    }
</style>