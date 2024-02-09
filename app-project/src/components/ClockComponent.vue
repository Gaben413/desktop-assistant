<template>
    <div class="time-slot">
        <h1>Testing this out</h1>
        <h2>{{ time_data.time_name }}</h2>
        <h3>{{ time_data.time_zone }}</h3>
        <p>{{ time_data.time }}</p>
    </div>
</template>

<script>
    export default{
        name: "ClockComponent",
        data(){
            return{
                time_data: {
                    time_name: "London",
                    time_zone: "Europe/London",
                    time: "00:00"
                },
                interval: null
            }
        },
        created() {

            this.interval = setInterval(() => {
                let nz_date_string = new Date().toLocaleString("pt-BR", { timeZone: this.time_data.time_zone });

                this.time_data.time = nz_date_string
            }, 1000)
        },
        unmounted(){
            clearInterval(this.interval);
        }
    }
</script>

<style>
    .time-slot{
        padding: 15px;
        border-style: dashed;
        border-color: black;

        text-align: center;
    }
</style>