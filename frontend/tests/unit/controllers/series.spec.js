import { shallowMount } from '@vue/test-utils';
import Series from '@/controllers/series.vue';
import Paginator from '@/components/paginator.vue';
import moxios from 'moxios';

import seriesResponse from '@/../mocks/seriesResponse0';

'use strict';

describe('Series.vue', () => {

    beforeEach( () => {
        // import and pass your custom axios instance to this method
        moxios.install();
    });
    afterEach( () => {
        // import and pass your custom axios instance to this method
        moxios.uninstall();
    });

    it('renders', (done) => {
        const wrapper = shallowMount(Series);
        expect(wrapper.findAll('series-item-stub').length).toBe(0);
        moxios.wait(() => {
            const request = moxios.requests.mostRecent();
            request.respondWith({
              status: 200,
              response: seriesResponse,
            }).then(() => {
                expect(wrapper.findAll('series-item-stub').length).toBe(6);
                //wrapper.vm.$nextTick();
               // callback();
                done();
            });//.finally(done);
        });

    });

    it('reloads on new page', (done) => {
        const wrapper = shallowMount(Series);
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

        const wrapper = shallowMount(Series);
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
