import { shallowMount } from '@vue/test-utils';
import Series from '@/controllers/series.vue';
import Paginator from '@/components/paginator.vue';
import moxios from 'moxios';

import seriesResponse from '@/../mocks/seriesResponse0';

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
        moxios.wait(() => {
            const request = moxios.requests.mostRecent();
            request.respondWith({
              status: 200,
              response: seriesResponse,
            }).then(() => {
              expect(wrapper.html()).toBe(`<div><h2>Series</h2> <ul><li>
          test series 1
      </li><li>
          test series 2
      </li><li>
          test series 3
      </li><li>
          test series 4
      </li><li>
          test series 5
      </li><li>
          test series 6
      </li></ul> <paginator-stub totalitems="11" pagesize="10" page="1"></paginator-stub></div>`);
              done();
            });
        });

    });

    it('reloads on new page', (done) => {
        const wrapper = shallowMount(Series);
        wrapper.find(Paginator).vm.$emit('paginator-page-change', 2);
        moxios.wait(() => {
            const request = moxios.requests.mostRecent();
            expect( request.config.params ).toEqual({ page: 1 });
            done();
        });
    });

});
