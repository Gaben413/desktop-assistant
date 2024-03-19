<template>
    <div :class="{ shake: disabled }">
        <div id="timer-container">
            <button class="timer-play-pause-buttons" @click="pause_timer">
                <img src="../assets/timer/play.svg" v-if="is_paused" class="pause-play-images">
                <img src="../assets/timer/pause.svg" class="pause-play-images" v-else>
            </button>
            <p class="timer-display">{{ timer_display }} | <span v-if="get_up">Get Up</span> <span v-else>Sit Down</span> </p>
            <button class="timer-buttons" @click="set_timer_sd">Sit Down</button>
            <button class="timer-buttons" @click="set_timer_su">Stand Up</button>
            <!--
            <button class="timer-buttons" @click="set_timer_test">Test</button>
            -->
            
        </div>
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
                disabled: false,
                continueShaking: true,
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
                }else if(this.timer_counter == 0 && this.continueShaking){
                    this.warn();
                    this.continueShaking = false;
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
                this.continueShaking = true;
            },
            set_timer_su(){
                this.timer_counter = 900;
                this.get_up = true;
                this.continueShaking = true;
            },
            set_timer_test(){
                this.timer_counter = 2;
                this.get_up = true;
                this.continueShaking = true;
            },
            warn(){
                this.disabled = true;
                setTimeout(()=>{
                    this.disabled = false;
                }, 1500);
            }
        }
    }
</script>

<style>
    #timer-container{
        display: grid;
        grid-template-columns: auto auto auto auto;

        justify-content: center;

        padding: 25px 0px;
        margin: 5px;

        border-style: groove;
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
    .timer-play-pause-buttons{
        height: 25px;
        width: 25px;
        padding: 2px 0px 0px 0px;

        margin: 0px 5px;

        background-color: white;

        box-shadow: none;
    }

    .pause-play-images{
        height: 20px;
        width: 20px;
    }

    .shake {
        animation: shake 0.82s cubic-bezier(0.36, 0.07, 0.19, 0.97) both;
        transform: translate3d(0, 0, 0);
    }

    @keyframes shake {
        10%,
        90% {
            transform: translate3d(-1px, 0, 0);
        }

        20%,
        80% {
            transform: translate3d(2px, 0, 0);
        }

        30%,
        50%,
        70% {
            transform: translate3d(-4px, 0, 0);
        }

        40%,
        60% {
            transform: translate3d(4px, 0, 0);
        }

        from {
            background-color: red;
        }
        to {
            background-color: whtie;
        }
    }
</style>