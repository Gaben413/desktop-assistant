<template>
    <div id="w-grid">
        <div id="season">
            <p class="season-p" v-if="season()==1">SPRING</p>
            <p class="season-p" v-else-if="season()==2">SUMMER</p>
            <p class="season-p" v-else-if="season()==3">AUTUMN</p>
            <p class="season-p" v-else-if="season()==4">WINTER</p>
        </div>
        <div id="moon">
            MOON
        </div>
        <div id="temperature">
            TEMPERATURE
        </div>
    </div>
    
</template>

<script setup>
    let now_date = new Date();
    let current_month = now_date.getMonth()+1;

    console.log(current_month)
    
    //[Meteorological Seasons, Oppositive Seasons]
    let spring_months = [[3, 4, 5], [9, 10, 11]];
    let summer_months = [[6, 7, 8], [12, 1, 2]];
    let autumn_months = [[9, 10, 11], [3, 4, 5]];
    let winter_months = [[12, 1, 2], [6, 7, 8]];

    let season = function(){
        let rotation = 1;

        if (spring_months[rotation].includes(current_month)){
            return 1;
        } else if (summer_months[rotation].includes(current_month)){
            return 2;
        } else if (autumn_months[rotation].includes(current_month)){
            return 3;
        } else if (winter_months[rotation].includes(current_month)){
            return 4;
        } 
    };

    console.log(season())

</script>

<style scoped>
    #w-grid{
        display: grid;

        grid-template-columns: auto auto auto;

        justify-content: center;

        justify-content: space-between;
    }
    .season-p{
        margin: 0;
    }
</style>