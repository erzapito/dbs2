import { shallowMount } from '@vue/test-utils';
import Wanted from '@/controllers/wanted.vue';
import Paginator from '@/components/paginator.vue';
import moxios from 'moxios';

import wantedResponse from '@/../mocks/wantedResponse';

describe('Wanted.vue', () => {

    beforeEach( () => {
        // import and pass your custom axios instance to this method
        moxios.install();
    });
    afterEach( () => {
        // import and pass your custom axios instance to this method
        moxios.uninstall();
    });

    it('renders', (done) => {
        const wrapper = shallowMount(Wanted);
        moxios.wait(() => {
            const request = moxios.requests.mostRecent();
            request.respondWith({
              status: 200,
              response: wantedResponse,
            }).then(() => {
                expect(wrapper.findAll('wanted-item-stub').length).toBe(6);
                done();
            });
        });

    });

});
