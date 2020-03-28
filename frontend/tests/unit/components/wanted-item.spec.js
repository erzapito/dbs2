import { shallowMount } from '@vue/test-utils';
import WantedItem from '@/components/wanted-item.vue';

describe('wanted-item.vue', () => {

    it('renders', (done) => {
        const wrapper = shallowMount(WantedItem,{
            propsData: {
                'item' : {
                    "id": 1,
                    "artist": "2",
                    "disc": "3",
                    "weeks": "4",
                    "done": true,
                },
            }
        });
      expect(wrapper.html()).toBe(`<tr class="wanted-item">
  <td class="id">1</td>
  <td class="artist">2</td>
  <td class="disc">3</td>
  <td class="weeks">4</td>
  <td class="done">true</td>
</tr>`);
      done();
    });


});
