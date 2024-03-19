<template>
    <div :style="{'height': '20px', 'width': '100%', 'background-color': bar_bg_color, 'position':'relative', 'border-radius': '5px',}">
        <div :style="{
            'height': '20px',
            'width': get_storage_percentage(current_ammount,total_ammount),
            'border-radius': '5px',
            'background-color': bar_color,
            'position':'absolute',
            'top': 0,
            'left': 0,
            'z-index': 1
            }"></div>
        <p :style="{
            'height': '20px', 
            'width': '100%',
            'text-align': 'right',
            'color': 'white',
            'position':'absolute',
            'top': 0,
            'right': '5px', /* Might need to delete this if it messes with the paragraph placement */
            'z-index': 2,
            'margin': 0,
        }">{{ get_storage_percentage(current_ammount,total_ammount) }}</p>
    </div>
</template>

<script>
    export default{
        props: {
            current_ammount: Number,
            total_ammount: Number,
            bar_color: String,
            bar_bg_color: String,
        },
        methods:{
            format_byte(input_bytes){
                //console.log(`INPUT: ${input_bytes} | C1: ${1/(1e-6)} | C2: ${1/(1e-9)}`);
                if(input_bytes < (1/1e-6)){
                    return `${input_bytes} BYTES`;
                }if(input_bytes >= (1/1e-6) && input_bytes < (1/1e-9)){
                    return `${(input_bytes * 1e-6).toFixed(2)} MB`;
                }else if(input_bytes >= (1/1e-9) && input_bytes < (1/1e-12)){
                    return `${(input_bytes * 1e-9).toFixed(2)} GB`;
                }else if(input_bytes >= (1/1e-12)){
                    return `${(input_bytes * 1e-12).toFixed(2)} TB`;
                }
            },
            get_storage_percentage(current_ammount, max_ammount){
                return `${((100*current_ammount)/max_ammount).toFixed(2)}%`;
            }
        }
    }
    
</script>