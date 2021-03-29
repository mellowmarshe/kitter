<template>
    <div :key="post.id" v-for="post in getPosts">
        <Post :post="post" />
    </div>
</template>

<script>
    import { mapGetters, mapActions } from "vuex";
    import Post from './Post'

    export default {
        name: 'Posts',
        components: {
            Post,
        },
        data() {
            return {
                posts: [],
            }
        },
        computed: mapGetters(["getPosts"]),
        methods: {
            ...mapActions(["fetchPosts"])
        },
        created() {
            this.fetchPosts();
        }
    }
</script>