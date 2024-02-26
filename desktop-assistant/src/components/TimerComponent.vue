<template>
    <div id="timer-container">
        <button class="timer-buttons" @click="pause_timer"><span v-if="is_paused">></span><span v-else>||</span></button>
        <p class="timer-display">{{ timer_display }} | <span v-if="get_up">Get Up</span> <span v-else>Sit Down</span> </p>
        <button class="timer-buttons" @click="set_timer_sd">Sit Down</button>
        <button class="timer-buttons" @click="set_timer_su">Stand Up</button>
    </div>
</template>

<script>
    export default {
        data(){
            return{
                timer_counter: 0,
                timer_display: "00:00:00",
                is_paused: false,
                get_up: false,
                interval: null,
            }
        },
        mounted(){
            this.timer_counter = 7200;

            this.interval = setInterval(()=>{
                if(!this.is_paused && this.timer_counter > 0){
                    this.timer_counter--;

                    let seconds = this.timer_counter%60;
                    let minutes = Math.floor(this.timer_counter/60);
                    let hours = Math.floor(minutes/60);

                    this.timer_display = `${hours}:${minutes%60}:${seconds}`;
                }
            }, 1000);
        },
        unmounted(){
            clearInterval(this.interval);
        },
        methods:{
            pause_timer(){
                this.is_paused = !this.is_paused;
            },
            set_timer_sd(){
                this.timer_counter = 7200;
                this.get_up = false;
            },
            set_timer_su(){
                this.timer_counter = 900;
                this.get_up = true;
            }
        }
    }
</script>

<style>
    #timer-container{
        display: grid;
        grid-template-columns: auto auto auto auto;

        justify-content: center;

        margin: 5px;
    }

    .timer-display{
        margin: 0px;
        padding: 0px;
    }

    .timer-buttons{
        height: 25px;
        padding: 0px 5px;

        margin: 0px 5px;

        background-color: white;
    }
</style>