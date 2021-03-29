<template>
    <div id="errors"></div>
    <form @submit="onSubmit" class="box mt-15" id="login-form">

        <div class="field">
            <label class="label">Username</label>
            <div class="control">
                <input class="input" v-model="username" name="username" type="text" placeholder="">
            </div>
            <small>Usernames must be between 3 and 32 characters. They can contain 1-9, a-Z, . and \_. They must
                start
                with a-Z.</small>
        </div>
        <div class="field">
            <label class="label">Email</label>
            <div class="control">
                <input class="input" v-model="email" name="email" type="email" placeholder="">
            </div>
        </div>
        <div class="field">
            <label class="label">Password</label>
            <div class="control">
                <input class="input" v-model="password" name="password" type="password" placeholder="">
            </div>
            <small>Password must be atleast 8 characters with an uppercase, lowercase and number. Special
                characters are
                allowed.</small>
        </div>

        <input class="button is-black" type="submit" value="Login">
    </form>
</template>

<script>
    import axios from 'axios';

    export default {
        name: "Register_",
        components: {},
        methods: {
            async onSubmit(e) {
                e.preventDefault()

                const login = {
                    username: this.username,
                    password: this.password,
                    email: this.email,
                }

                const res = await axios.post('http://localhost:8083/api/user/register', JSON.stringify(login), {
                    headers: {
                        'Content-type': 'application/json',
                    },
                })
                console.log(res);
                this.$router.push('/login');
            },
        },
    };
</script>

<style lang="scss" scoped>

</style>