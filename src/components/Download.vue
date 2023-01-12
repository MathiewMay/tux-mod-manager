<template>
<div class="download">
    <i v-if="install_status == 3" class="fa fa-duotone fa-check"></i>
    <i v-if="install_status == 2" class="fa fa-duotone fa-download"></i>
    <i v-if="install_status == 0 || install_status == 1" class="fa fa-duotone fa-arrows-rotate fa-spin"></i>
    <div class="download-name">{{ filename }}</div>
    <div v-if="install_status == 0 || install_status == 1" class="download-progress">
        <div v-if="install_status == 0" class="progress-label">{{ progress }}%</div>
        <div v-if="install_status == 1" class="progress-label">{{ progress }}</div>
        <progress class="progress" max="100" v-if="install_status == 0" :value="progress"></progress>
        <progress class="progress" max="100" v-if="install_status == 1"></progress>
    </div>
    <div class="download-options">
        <button v-if="install_status == 0 || install_status == 1">Cancel</button>
        <button v-if="install_status == 2 || install_status == 3">Remove</button>
        <button v-if="install_status == 2">Install</button>
    </div>
</div>
</template>

<script>

export default {
    props: ['filename', 'install_status', 'progress']
}
</script>

<style lang="scss" scoped>
.download {
    display: flex;
    flex-direction: row;
    padding: 5px;
    border-bottom: 1px solid rgba(255,255,255,8%);
    &:last-child {
        border-bottom: none;
    }
    .fa {
        min-width: 30px;
        width: 30px; line-height: 30px;
        text-align: center;
        margin-right: 10px;
        color: white;
        font-size: x-large;
    }
    .download-name {
        line-height: 30px;
        height: 30px;
        overflow: hidden;
        margin-right: auto;
    }
    .download-progress {
        display: flex;
        margin-left: 10px;
        // flex: 1 1 auto;
        .progress-label {
            line-height: 30px;
        }
        progress {
            flex: 1 1 auto;
            height: 30px;
            width: 200px;
            margin: 0;
            margin-inline: 10px;
            appearance: none;
            border-radius: 5px;
            overflow: hidden;
            position: relative;
            &::-webkit-progress-bar {
                background-color: rgba(255,255,255,7%);
                // box-shadow: inset 0 0 15px #00000055;
                box-shadow: 0 2px 5px rgba(0, 0, 0, 0.25) inset;
            }
            &::-webkit-progress-value {
                // background-color: hsl(140, 74%, 15%);
                //This background is taken from: https://css-tricks.com/html5-progress-element/ (at 22:38 06.06.2022)
                background-image:
                -webkit-linear-gradient(
                    -45deg, 
                    transparent 33%,
                  rgba(0, 0, 0, .1) 33%, 
                  rgba(0,0, 0, .1) 66%,
                    transparent 66%
                ),
                -webkit-linear-gradient(
                    top, 
                  rgba(255, 255, 255, .25), 
                  rgba(0, 0, 0, .25),
                ),
                -webkit-linear-gradient(
                    left, rgb(0, 204, 126), rgb(143, 255, 68)
                );
                background-size: 60px 30px, 100% 100%, 100% 100%;
                box-shadow: 0 0 15px #00000088;
                position: relative;
                -webkit-animation: animate-stripes 2s linear infinite;
                animation: animate-stripes 2s linear infinite;
                border-radius: 5px;
            }
        }
        progress:not([value]) {
            &::-webkit-progress-value {
                width: 100%;
            }
        }
    }
    .download-options {
        button {
            margin: 0;
            margin-right: 10px;
            height: 30px;
            &:last-child {
                margin-right: 0;
            }
        }
    }
}

@-webkit-keyframes animate-stripes {
    // 0% { background-position: 0 0px, 0 0, 0 0; }
    100% { background-position: -60px 0px, 0 0, 0 0; }
}

@keyframes animate-stripes {
    // 0% { background-position: 0 0px, 0 0, 0 0; }
    100% { background-position: -60px 0px, 0 0, 0 0; }
}
</style>