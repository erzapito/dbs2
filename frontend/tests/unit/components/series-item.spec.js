import { shallowMount } from '@vue/test-utils';
import SeriesItem from '@/components/series-item.vue';

describe('series-item.vue', () => {

    it('renders', (done) => {
        const wrapper = shallowMount(SeriesItem,{
            propsData: {
                'item' : {
                    "id": 1,
                    "capitulos": "1-24",
                    "categoria": "Descargado",
                    "fansub": "HorribleSubs",
                    "idioma": "es",
                    "name": "test series 1"
                },
            }
        });
      expect(wrapper.html()).toBe(`<div class="series-item">
    1
    <a href="javascript:void(0)">test series 1</a> <!----></div>`);
      done();
    });

    it('shows form after click', async (done) => {
        const wrapper = shallowMount(SeriesItem,{
            propsData: {
                'item' : {
                    "id": 1,
                    "capitulos": "1-24",
                    "categoria": "Descargado",
                    "fansub": "HorribleSubs",
                    "idioma": "es",
                    "name": "test series 1"
                },
            }
        });
        const link = wrapper.find('a');
        link.trigger('click');

        await wrapper.vm.$nextTick()

      expect(wrapper.html()).toBe(`<div class="series-item">
    1
    <a href="javascript:void(0)">test series 1</a> <series-form-stub item="[object Object]"></series-form-stub></div>`);
      done();
    });

});
