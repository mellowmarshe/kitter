<template>
    <div class="box is-dark post mb-4">
        <article class="media">
            <div class="media-content">
                <div class="content">
                    <p>
                        <strong>{{ post.author }}</strong>
                        <small>{{ new Date(post.timestamp).toLocaleString() }}</small>
                        <br>
                        <span style="white-space: pre-wrap;">{{ post.content }}</span>
                    </p>
                </div>
            </div>
        </article>
    </div>


</template>

<script>
    export default {
        name: 'Post',
        props: {
            post: Object,
        },
    }
</script>