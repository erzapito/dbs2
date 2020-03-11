import { shallowMount } from '@vue/test-utils';
import Music from '@/controllers/music.vue';
import Paginator from '@/components/paginator.vue';
import moxios from 'moxios';

import musicResponse from '@/../mocks/musicResponse0';

describe('Music.vue', () => {

    beforeEach( () => {
        // import and pass your custom axios instance to this method
        moxios.install();
    });
    afterEach( () => {
        // import and pass your custom axios instance to this method
        moxios.uninstall();
    });

    it('renders', (done) => {
        const wrapper = shallowMount(Music);
        moxios.wait(() => {
            const request = moxios.requests.mostRecent();
            request.respondWith({
              status: 200,
              response: musicResponse,
            }).then(() => {
                expect(wrapper.findAll('music-item-stub').length).toBe(6);
                done();
            });
        });

    });

    it('reloads on new page', (done) => {
        const wrapper = shallowMount(Music);
        wrapper.find(Paginator).vm.$emit('paginator-page-change', 2);
        moxios.wait(() => {
            const request = moxios.requests.mostRecent();
            expect( request.config.params ).toEqual({
                page: 1,
                search: '',
            });
            done();
        });
    });

    it('reloads after search edit', (done) => {
        jest.useFakeTimers();

        const wrapper = shallowMount(Music);
        const searchInput = wrapper.find(".search input");
        searchInput.trigger('keyup');
        expect(clearTimeout).toHaveBeenCalledTimes(0);
        jest.runAllTimers();
        expect(setTimeout).toHaveBeenLastCalledWith(expect.any(Function), 500);
        searchInput.trigger('keyup');
        expect(clearTimeout).toHaveBeenCalledTimes(1);
        expect(setTimeout).toHaveBeenLastCalledWith(expect.any(Function), 500);
        done();
    });

});
