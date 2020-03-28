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
        expect(wrapper.find('.id').text()).toBe("1");
        expect(wrapper.find('a.toggle').text()).toBe('test series 1');
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
        expect(wrapper.findAll('series-form-stub').exists()).toBe(false);
        const link = wrapper.find('a.toggle');
        link.trigger('click');
        await wrapper.vm.$nextTick()
        expect(wrapper.findAll('series-form-stub').exists()).toBe(true);
        done();
    });

});
