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
      expect(wrapper.html()).toBe(`<div class="music-item">
    1
    <a href="javascript:void(0)">ARTIST - DISC</a> <!----></div>`);
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
        const link = wrapper.find('a');
        link.trigger('click');

        await wrapper.vm.$nextTick()

      expect(wrapper.html()).toBe(`<div class="music-item">
    1
    <a href="javascript:void(0)">ARTIST - DISC</a> <music-form-stub item="[object Object]"></music-form-stub></div>`);
      done();
    });

});
