<template>
    <div class="home">
        <div class="columns">
            <div class="column is-three-fifths pr-0">
                <Posts :posts="posts"></Posts>
            </div>
            <div class="column ml-4 pl-0">
                <div>
                    <a class="button is-black" href="/auth/logout">
                        <strong>{{ this.$store.state.username }}</strong>
                    </a>
                    <a class="button is-black ml-4" value="post" id="post-button" href="#">
                        <i class="fa fa-plus"></i>
                    </a>
                </div>

                <div class="box mt-4">
                    <strong>More to come here!</strong>
                    <br>
                    <strong>Maybe tags....</strong>
                    <br>
                    <strong>Or sneak peaks at features....</strong>
                    <br>
                    <strong>Such as users most recent post...</strong>
                </div>
            </div>
        </div>
    </div>
</template>

<script>
    import Posts from '../components/Posts.vue';

    export default {
        name: "Home",
        components: {
            Posts
        }
    };
</script>