import { redirect } from '@sveltejs/kit';

/** @type {import('./$types').PageLoad} */
export function load({ params }) {

    if (true) { // TODO: implement authentication logic, please tell me you implemented argon2 by now.
        throw redirect(308, '/login')
    }
}