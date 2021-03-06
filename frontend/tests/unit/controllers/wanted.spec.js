import { shallowMount } from '@vue/test-utils';
import Wanted from '@/controllers/wanted.vue';
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
            expect(request.config.url).toEqual('/api/wanted');
            expect(request.config.method).toEqual('get');
            request.respondWith({
              status: 200,
              response: wantedResponse,
            }).then(() => {
                expect(wrapper.findAll('wanted-item-stub').length).toBe(6);
                done();
            });
        });
    });

    it('reloads after wanted marked', (done) => {
        const wrapper = shallowMount(Wanted);
        moxios.wait(() => {
            const request = moxios.requests.mostRecent();
            expect(request.config.url).toEqual('/api/wanted');
            expect(request.config.method).toEqual('get');
            request.respondWith({
              status: 200,
              response: wantedResponse,
            }).then(() => {
                expect(wrapper.findAll('wanted-item-stub').length).toBe(6);
                wrapper.find('wanted-item-stub').vm.$emit('reload', null);
                moxios.wait(() => {
                    const request = moxios.requests.mostRecent();
                    expect(request.config.url).toEqual('/api/wanted');
                    expect(request.config.method).toEqual('get');
                    done();
                });
            });
        });
    });

});
