import { shallowMount } from '@vue/test-utils';
import MusicItem from '@/components/music-item.vue';

describe('music-item.vue', () => {

    it('renders', (done) => {
        const wrapper = shallowMount(MusicItem,{
            propsData: {
                'item' : {
                    "id": 1,
                    "artist": "ARTIST",
                    "disc": "DISC",
                },
            }
        });
        expect(wrapper.find('.id').text()).toBe("1");
        expect(wrapper.find('a.toggle').text()).toBe('ARTIST - DISC');
        done();
      done();
    });

    it('shows form after click', async (done) => {
        const wrapper = shallowMount(MusicItem,{
            propsData: {
                'item' : {
                    "id": 1,
                    "artist": "ARTIST",
                    "disc": "DISC",
                },
            }
        });
        expect(wrapper.findAll('music-form-stub').exists()).toBe(false);
        const link = wrapper.find('a.toggle');
        link.trigger('click');
        await wrapper.vm.$nextTick()
        expect(wrapper.findAll('music-form-stub').exists()).toBe(true);
        done();
    });

});
